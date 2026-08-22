#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
use tauri::{Manager, Emitter};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::UpdaterExt;

#[cfg(windows)]
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, KEYEVENTF_SCANCODE,
    VIRTUAL_KEY, VK_MENU, VK_TAB, VK_SHIFT, VK_CONTROL, VK_LMENU, VK_RETURN, VK_HOME,
    VK_SPACE,
    VkKeyScanW, MapVirtualKeyW, MAP_VIRTUAL_KEY_TYPE, ToUnicodeEx, GetKeyboardLayout,
};


#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx,
    PeekMessageW, TranslateMessage, DispatchMessageW,
    WH_MOUSE_LL, PM_REMOVE,
};

#[cfg(windows)]
use windows::Win32::Foundation::{WPARAM, LPARAM, LRESULT};

#[cfg(windows)]
static CLICK_COUNT: AtomicU32 = AtomicU32::new(0);

/// Simulate ALT+TAB to switch to the next window.
#[cfg(windows)]
fn alt_tab() {
    let mut inputs: [INPUT; 4] = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_MENU, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_TAB, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_TAB, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_MENU, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
    ];
    unsafe { SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32); }
}

/// Low-level mouse hook callback. Increments CLICK_COUNT when the left button goes down.
#[cfg(windows)]
unsafe extern "system" fn mouse_hook_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    // WM_LBUTTONDOWN = 0x0201
    if n_code >= 0 && w_param.0 == 0x0201 {
        CLICK_COUNT.fetch_add(1, Ordering::SeqCst);
    }
    CallNextHookEx(None, n_code, w_param, l_param)
}

/// Wait for the user to left-click `required_clicks` times anywhere on screen.
/// Installs a low-level mouse hook, pumps messages until enough clicks are
/// detected, then removes the hook. Times out after `timeout` duration.
/// The hook only observes clicks (CallNextHookEx) so they still reach the
/// target app — a genuine double-click selects a word naturally.
#[cfg(windows)]
fn wait_for_user_click(required_clicks: u32, timeout: Duration) -> Result<(), String> {
    let required = required_clicks.max(1);
    CLICK_COUNT.store(0, Ordering::SeqCst);

    let hook = unsafe {
        SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), None, 0)
            .map_err(|e| format!("Failed to install mouse hook: {}", e))?
    };

    let start = std::time::Instant::now();
    let mut msg = unsafe { std::mem::zeroed() };

    while CLICK_COUNT.load(Ordering::SeqCst) < required {
        if start.elapsed() > timeout {
            unsafe { let _ = UnhookWindowsHookEx(hook); }
            return Err("Timeout waiting for click".to_string());
        }

        unsafe {
            if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    unsafe { let _ = UnhookWindowsHookEx(hook); }
    Ok(())
}

/// How a newline is typed into the target: Enter, plus the optional auto-indent cleanup.
///
/// `settle_ms` is the pause after Enter that lets the target app actually process the
/// keystroke and insert its auto-indent before anything else is sent. It is derived from
/// the configured typing delay instead of a fixed constant — a slow target (deep VDI, busy
/// editor) is exactly the case where a hardcoded value is too short and the follow-up keys
/// still land on the *previous* line.
#[derive(Clone, Copy)]
struct EnterOpts {
    /// Drop leading whitespace the target inserted automatically after Enter, so the
    /// indentation carried by the pasted text is not added on top of it.
    clear_indent: bool,
    /// Pause after Enter before the indent cleanup runs (ms).
    settle_ms: u64,
}

impl EnterOpts {
    /// Derive the post-Enter settle time from the configured per-character typing delay.
    /// Scales with how slow the user has told us the target is, with a sane floor/ceiling.
    fn from_typing_delay(clear_indent: bool, typing_delay_ms: u64) -> Self {
        let settle_ms = (typing_delay_ms.saturating_mul(4)).clamp(60, 500);
        EnterOpts { clear_indent, settle_ms }
    }
}

/// Simulate pressing the Enter key, optionally clearing auto-indent on the new line.
///
/// Cleanup is Shift+Home alone: it selects *backwards* from the cursor to the line start,
/// which can only ever cover whitespace the target just inserted. The selection is left
/// standing — the next typed character overwrites it, exactly as it would for a human.
/// No Delete is sent, because on a line that was never auto-indented the selection is
/// empty and Delete would eat the character after the cursor instead. Typing over an
/// empty selection is simply an insert, so the no-auto-indent case stays harmless.
#[cfg(windows)]
fn send_enter(opts: EnterOpts) {
    use windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_EXTENDEDKEY;

    let scan_ret = unsafe { MapVirtualKeyW(VK_RETURN.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;

    // Enter down+up
    let mut enter_inputs: [INPUT; 2] = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_RETURN, wScan: scan_ret, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_RETURN, wScan: scan_ret, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
    ];
    unsafe { SendInput(&mut enter_inputs, std::mem::size_of::<INPUT>() as i32); }

    // Always let the target process the newline before sending anything else, so the next
    // character can't race ahead of the line break.
    thread::sleep(Duration::from_millis(opts.settle_ms));

    if !opts.clear_indent {
        return;
    }

    let scan_home = unsafe { MapVirtualKeyW(VK_HOME.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let scan_shift = unsafe { MapVirtualKeyW(VK_SHIFT.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;

    let ext = KEYEVENTF_EXTENDEDKEY;
    let ext_up = KEYBD_EVENT_FLAGS(KEYEVENTF_EXTENDEDKEY.0 | KEYEVENTF_KEYUP.0);

    // Shift+Home — select from the cursor back to the line start (the auto-indent).
    // Home needs KEYEVENTF_EXTENDEDKEY to avoid being read as a Numpad key.
    let mut select_inputs: [INPUT; 4] = [
        // Shift down
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_SHIFT, wScan: scan_shift, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 } } },
        // Home down (extended, with Shift held = select to line start)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_HOME, wScan: scan_home, dwFlags: ext, time: 0, dwExtraInfo: 0 } } },
        // Home up (extended)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_HOME, wScan: scan_home, dwFlags: ext_up, time: 0, dwExtraInfo: 0 } } },
        // Shift up
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_SHIFT, wScan: scan_shift, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 } } },
    ];
    unsafe { SendInput(&mut select_inputs, std::mem::size_of::<INPUT>() as i32); }

    // Let the selection register before the next character overwrites it — same race as
    // above, one step later.
    thread::sleep(Duration::from_millis(opts.settle_ms / 2));
}

/// Send a real Tab keypress.
///
/// Tab is a control character (U+0009). Injected with KEYEVENTF_UNICODE it arrives as a
/// WM_CHAR that edit controls, browsers and most editors silently drop - which is why
/// tab-based indentation vanished in unicode mode while the vkey path, which maps '\t' to
/// the physical Tab key, reproduced it correctly.
#[cfg(windows)]
fn send_tab() {
    let scan_tab = unsafe { MapVirtualKeyW(VK_TAB.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let mut inputs: [INPUT; 2] = [
        make_key_input(VK_TAB, scan_tab, KEYEVENTF_SCANCODE),
        make_key_input(VK_TAB, scan_tab, KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0 | KEYEVENTF_KEYUP.0)),
    ];
    unsafe { SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32); }
}

/// Send a single Unicode character as a keypress via Windows SendInput.
#[cfg(windows)]
fn send_unicode_char(c: char, enter: EnterOpts) {
    // Newlines → send Enter keypress instead of Unicode control character
    if c == '\n' || c == '\r' {
        send_enter(enter);
        return;
    }
    // Tab -> real Tab keypress; as a Unicode control character the target drops it.
    if c == '\t' {
        send_tab();
        return;
    }
    let codes: Vec<u16> = c.encode_utf16(&mut [0u16; 2]).to_vec();
    for code in codes {
        let mut inputs: [INPUT; 2] = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0), wScan: code, dwFlags: KEYEVENTF_UNICODE, time: 0, dwExtraInfo: 0 },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: VIRTUAL_KEY(0), wScan: code, dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_UNICODE.0 | KEYEVENTF_KEYUP.0), time: 0, dwExtraInfo: 0 },
                },
            },
        ];
        unsafe { SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32); }
    }
}

/// EN-US keyboard layout mapping: returns (scancode, shift_state) for a character.
/// Uses hardcoded scancodes from the EN-US physical layout, bypassing MapVirtualKeyW
/// which would use the local (e.g. German) layout and produce wrong scancodes.
/// shift_state: bit 0 = Shift.
/// Returns None for characters not available on EN-US layout (e.g. ö, ä, ü, ß).
#[cfg(windows)]
fn enus_char_to_scancode(c: char) -> Option<(u16, u8)> {
    // Scancodes are hardware positions on a standard 101/102-key keyboard (Set 1).
    // These are layout-independent — the same physical key always has the same scancode.
    let (sc, shift): (u16, u8) = match c {
        // Row 1: number row (scancodes 0x02..0x0D)
        '1' => (0x02, 0), '!' => (0x02, 1),
        '2' => (0x03, 0), '@' => (0x03, 1),
        '3' => (0x04, 0), '#' => (0x04, 1),
        '4' => (0x05, 0), '$' => (0x05, 1),
        '5' => (0x06, 0), '%' => (0x06, 1),
        '6' => (0x07, 0), '^' => (0x07, 1),
        '7' => (0x08, 0), '&' => (0x08, 1),
        '8' => (0x09, 0), '*' => (0x09, 1),
        '9' => (0x0A, 0), '(' => (0x0A, 1),
        '0' => (0x0B, 0), ')' => (0x0B, 1),
        '-' => (0x0C, 0), '_' => (0x0C, 1),
        '=' => (0x0D, 0), '+' => (0x0D, 1),

        // Row 2: QWERTY row (scancodes 0x10..0x1B)
        'q' => (0x10, 0), 'Q' => (0x10, 1),
        'w' => (0x11, 0), 'W' => (0x11, 1),
        'e' => (0x12, 0), 'E' => (0x12, 1),
        'r' => (0x13, 0), 'R' => (0x13, 1),
        't' => (0x14, 0), 'T' => (0x14, 1),
        'y' => (0x15, 0), 'Y' => (0x15, 1),
        'u' => (0x16, 0), 'U' => (0x16, 1),
        'i' => (0x17, 0), 'I' => (0x17, 1),
        'o' => (0x18, 0), 'O' => (0x18, 1),
        'p' => (0x19, 0), 'P' => (0x19, 1),
        '[' => (0x1A, 0), '{' => (0x1A, 1),
        ']' => (0x1B, 0), '}' => (0x1B, 1),

        // Row 3: home row (scancodes 0x1E..0x28)
        'a' => (0x1E, 0), 'A' => (0x1E, 1),
        's' => (0x1F, 0), 'S' => (0x1F, 1),
        'd' => (0x20, 0), 'D' => (0x20, 1),
        'f' => (0x21, 0), 'F' => (0x21, 1),
        'g' => (0x22, 0), 'G' => (0x22, 1),
        'h' => (0x23, 0), 'H' => (0x23, 1),
        'j' => (0x24, 0), 'J' => (0x24, 1),
        'k' => (0x25, 0), 'K' => (0x25, 1),
        'l' => (0x26, 0), 'L' => (0x26, 1),
        ';' => (0x27, 0), ':' => (0x27, 1),
        '\'' => (0x28, 0), '"' => (0x28, 1),

        // Backtick/tilde (scancode 0x29)
        '`' => (0x29, 0), '~' => (0x29, 1),

        // Backslash/pipe (scancode 0x2B)
        '\\' => (0x2B, 0), '|' => (0x2B, 1),

        // Row 4: bottom row (scancodes 0x2C..0x35)
        'z' => (0x2C, 0), 'Z' => (0x2C, 1),
        'x' => (0x2D, 0), 'X' => (0x2D, 1),
        'c' => (0x2E, 0), 'C' => (0x2E, 1),
        'v' => (0x2F, 0), 'V' => (0x2F, 1),
        'b' => (0x30, 0), 'B' => (0x30, 1),
        'n' => (0x31, 0), 'N' => (0x31, 1),
        'm' => (0x32, 0), 'M' => (0x32, 1),
        ',' => (0x33, 0), '<' => (0x33, 1),
        '.' => (0x34, 0), '>' => (0x34, 1),
        '/' => (0x35, 0), '?' => (0x35, 1),

        // Special keys
        ' ' => (0x39, 0),   // Space
        '\t' => (0x0F, 0),  // Tab

        _ => return None,
    };
    Some((sc, shift))
}

/// Send a character targeting an EN-US keyboard layout on the remote side.
/// Uses hardcoded scancodes for the EN-US physical layout, completely bypassing
/// MapVirtualKeyW and VkKeyScanW, so the local system layout is irrelevant.
/// Falls back to Unicode mode for characters not on the EN-US layout (ö, ä, ü, ß, etc.).
#[cfg(windows)]
fn send_vkey_char_enus(c: char, key_delay_ms: u64, enter: EnterOpts) {
    if c == '\n' || c == '\r' {
        send_enter(enter);
        return;
    }

    let (scancode, shift_state) = match enus_char_to_scancode(c) {
        Some(m) => m,
        None => {
            send_unicode_char(c, enter);
            return;
        }
    };

    let needs_shift = (shift_state & 1) != 0;
    let intra_delay = Duration::from_millis(key_delay_ms);

    // Press Shift if needed
    if needs_shift {
        let scan_shift = 0x2Au16; // Left Shift scancode
        let mut input = [INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scan_shift,
                    dwFlags: KEYEVENTF_SCANCODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
        thread::sleep(intra_delay);
    }

    // Press and release the key using scancode only (no VK)
    let mut key_events: [INPUT; 2] = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scancode,
                    dwFlags: KEYEVENTF_SCANCODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scancode,
                    dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0 | KEYEVENTF_KEYUP.0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    unsafe { SendInput(&mut key_events, std::mem::size_of::<INPUT>() as i32); }

    // Release Shift if it was pressed
    if needs_shift {
        thread::sleep(intra_delay);
        let scan_shift = 0x2Au16;
        let mut input = [INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scan_shift,
                    dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0 | KEYEVENTF_KEYUP.0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }
}

/// Does pressing `vk` in `shift_state` arm a dead key on the local layout?
///
/// Dead keys (` ´ ^ ~ on German, French and most non-US layouts) do not emit a character.
/// They arm an accent and wait for the next key to decide whether the two combine. A dead
/// key at the very end of the text therefore never appears at all — that is why
/// "`Chat.txt`" lost its closing backtick while the opening one survived (the following
/// "C" does not combine, so the layout flushed the accent out in front of it).
///
/// wFlags bit 2 asks ToUnicodeEx not to mutate the layout's own dead-key state
/// (Windows 10 1607+), which keeps this a side-effect-free query.
#[cfg(windows)]
fn is_dead_key(vk: VIRTUAL_KEY, scancode: u16, shift_state: u16) -> bool {
    let mut key_state = [0u8; 256];
    if shift_state & 1 != 0 { key_state[VK_SHIFT.0 as usize] = 0x80; }
    if shift_state & 2 != 0 { key_state[VK_CONTROL.0 as usize] = 0x80; }
    if shift_state & 4 != 0 { key_state[VK_MENU.0 as usize] = 0x80; }

    let hkl = unsafe { GetKeyboardLayout(0) };
    let mut buf = [0u16; 8];
    let rc = unsafe {
        ToUnicodeEx(vk.0 as u32, scancode as u32, &key_state, &mut buf, 0x04, Some(hkl))
    };
    rc < 0
}

/// Press Space to release an armed dead key as its standalone accent character.
///
/// This is what a human does to type a bare ` or ^: hit the dead key, then Space. It also
/// guarantees the accent cannot silently combine with the next character we send.
#[cfg(windows)]
fn flush_dead_key(intra_delay: Duration) {
    thread::sleep(intra_delay);
    let scan_space = unsafe { MapVirtualKeyW(VK_SPACE.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let mut space_events: [INPUT; 2] = [
        make_key_input(VK_SPACE, scan_space, KEYEVENTF_SCANCODE),
        make_key_input(VK_SPACE, scan_space, KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0 | KEYEVENTF_KEYUP.0)),
    ];
    unsafe { SendInput(&mut space_events, std::mem::size_of::<INPUT>() as i32); }
}

/// Send a character by simulating real key presses via VkKeyScanW (compatible with VDI/Remote).
/// Maps the character to a virtual key code + modifier state, then sends scancode-based input.
/// Sends modifier and key events separately with small delays between them to ensure
/// nested remote sessions (RDP → VDI → Console) process each event correctly.
/// Falls back to Unicode mode for characters not in the current keyboard layout.
#[cfg(windows)]
fn send_vkey_char(c: char, key_delay_ms: u64, enter: EnterOpts) {
    // Newlines → send Enter keypress
    if c == '\n' || c == '\r' {
        send_enter(enter);
        return;
    }

    // Encode to UTF-16 and use VkKeyScanW to find the virtual key
    let mut buf = [0u16; 2];
    let encoded = c.encode_utf16(&mut buf);

    // VkKeyScanW only works with BMP characters (single u16)
    if encoded.len() != 1 {
        // Supplementary character — fall back to unicode mode
        send_unicode_char(c, enter);
        return;
    }

    let result = unsafe { VkKeyScanW(encoded[0]) };

    // VkKeyScanW returns -1 if the character can't be mapped
    if result == -1 {
        // Character not available in current keyboard layout — fall back to unicode
        send_unicode_char(c, enter);
        return;
    }

    let vk = VIRTUAL_KEY((result as u16) & 0xFF);
    let shift_state = ((result as u16) >> 8) & 0xFF;
    let needs_shift = (shift_state & 1) != 0;
    let needs_ctrl = (shift_state & 2) != 0;
    let needs_alt = (shift_state & 4) != 0;

    // Get the hardware scancode for this virtual key
    let scancode = unsafe { MapVirtualKeyW(vk.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;

    let intra_delay = Duration::from_millis(key_delay_ms);
    let has_modifiers = needs_shift || needs_ctrl || needs_alt;
    // Query before sending: this key may arm an accent instead of typing a character.
    let dead = is_dead_key(vk, scancode, shift_state);

    // Press modifiers one at a time with delays between them
    if needs_shift {
        let mut input = [make_key_input(VK_SHIFT, 0, KEYBD_EVENT_FLAGS(0))];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }
    if needs_ctrl {
        let mut input = [make_key_input(VK_CONTROL, 0, KEYBD_EVENT_FLAGS(0))];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }
    if needs_alt {
        let mut input = [make_key_input(VK_LMENU, 0, KEYBD_EVENT_FLAGS(0))];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }

    // Delay after modifiers to let remote sessions register the modifier state
    if has_modifiers {
        thread::sleep(intra_delay);
    }

    // Press and release the key (using scancode)
    let mut key_events: [INPUT; 2] = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: scancode,
                    dwFlags: KEYEVENTF_SCANCODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: scancode,
                    dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0 | KEYEVENTF_KEYUP.0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    unsafe { SendInput(&mut key_events, std::mem::size_of::<INPUT>() as i32); }

    // Delay before releasing modifiers to ensure key event was processed with modifier held
    if has_modifiers {
        thread::sleep(intra_delay);
    }

    // Release modifiers (reverse order)
    if needs_alt {
        let mut input = [make_key_input(VK_LMENU, 0, KEYEVENTF_KEYUP)];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }
    if needs_ctrl {
        let mut input = [make_key_input(VK_CONTROL, 0, KEYEVENTF_KEYUP)];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }
    if needs_shift {
        let mut input = [make_key_input(VK_SHIFT, 0, KEYEVENTF_KEYUP)];
        unsafe { SendInput(&mut input, std::mem::size_of::<INPUT>() as i32); }
    }

    // Dead key armed an accent instead of typing it — Space commits it as a real character.
    if dead {
        flush_dead_key(intra_delay);
    }
}

/// Helper to build a simple key INPUT event.
#[cfg(windows)]
fn make_key_input(vk: VIRTUAL_KEY, scan: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Tauri command: check if Accessibility permission is granted.
/// Always returns true on Windows.
#[tauri::command]
fn check_accessibility_permission() -> bool {
    true
}

/// Tauri command: open accessibility settings. No-op on Windows.
#[tauri::command]
fn open_accessibility_settings() -> Result<(), String> {
    Ok(())
}

/// Tauri command: open or focus the settings window.
/// Uses run_on_main_thread to avoid deadlocking when called from a webview invoke.
#[tauri::command]
async fn open_settings(app_handle: tauri::AppHandle) -> Result<(), String> {
    let aot = always_on_top_enabled(&app_handle);
    if let Some(win) = app_handle.get_webview_window("settings") {
        let _ = win.show();
        focus_child_window(&win, aot);
        return Ok(());
    }
    let handle = app_handle.clone();
    app_handle.run_on_main_thread(move || {
        if let Ok(win) = tauri::WebviewWindowBuilder::new(
            &handle,
            "settings",
            tauri::WebviewUrl::App("settings.html".into()),
        )
        .title("Settings — Virtual Copy Paste")
        .inner_size(480.0, 450.0)
        .resizable(true)
        .visible(false)
        .build()
        {
            position_child_near_main(&handle, &win);
            let _ = win.show();
            focus_child_window(&win, aot);
        }
    }).map_err(|e| format!("Failed to open settings: {}", e))?;
    Ok(())
}

/// Tauri command: return the app version from tauri.conf.json.
#[tauri::command]
fn get_version(app_handle: tauri::AppHandle) -> String {
    app_handle.config().version.clone().unwrap_or_else(|| "unknown".to_string())
}

/// Read the Windows system proxy from the registry (Internet Settings).
/// Returns the proxy URL (e.g. "http://proxy:8080") or None if no proxy is configured.
#[cfg(windows)]
fn get_windows_system_proxy() -> Option<String> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    let output = Command::new("reg")
        .args(["query", r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings", "/v", "ProxyEnable"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .ok()?;
    let enable_str = String::from_utf8_lossy(&output.stdout);
    // ProxyEnable REG_DWORD 0x1 means proxy is active
    if !enable_str.contains("0x1") {
        return None;
    }
    let output = Command::new("reg")
        .args(["query", r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings", "/v", "ProxyServer"])
        .creation_flags(0x08000000)
        .output()
        .ok()?;
    let proxy_str = String::from_utf8_lossy(&output.stdout);
    for line in proxy_str.lines() {
        if line.contains("ProxyServer") {
            let value = line.split_whitespace().last()?;
            if value.is_empty() {
                return None;
            }
            // If the proxy value contains "=" it's per-protocol (e.g. "http=proxy:80;https=proxy:443")
            if value.contains('=') {
                for part in value.split(';') {
                    if part.starts_with("https=") {
                        let addr = part.strip_prefix("https=")?;
                        return if addr.contains("://") {
                            Some(addr.to_string())
                        } else {
                            Some(format!("http://{}", addr))
                        };
                    }
                }
                // Fall back to http proxy if no https-specific one
                for part in value.split(';') {
                    if part.starts_with("http=") {
                        let addr = part.strip_prefix("http=")?;
                        return if addr.contains("://") {
                            Some(addr.to_string())
                        } else {
                            Some(format!("http://{}", addr))
                        };
                    }
                }
                return None;
            }
            // Simple proxy (e.g. "proxy:8080")
            return if value.contains("://") {
                Some(value.to_string())
            } else {
                Some(format!("http://{}", value))
            };
        }
    }
    None
}

/// Tauri command: return the system proxy URL, if any.
#[tauri::command]
fn get_system_proxy() -> Option<String> {
    // Check environment variables first
    if let Ok(proxy) = std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("https_proxy")) {
        if !proxy.is_empty() {
            return Some(proxy);
        }
    }
    if let Ok(proxy) = std::env::var("ALL_PROXY").or_else(|_| std::env::var("all_proxy")) {
        if !proxy.is_empty() {
            return Some(proxy);
        }
    }
    #[cfg(windows)]
    {
        return get_windows_system_proxy();
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Tauri command: open a URL in the default browser.
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows::core::PCWSTR;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        let url_wide: Vec<u16> = url.encode_utf16().chain(std::iter::once(0)).collect();
        let operation: Vec<u16> = "open".encode_utf16().chain(std::iter::once(0)).collect();

        let result = unsafe {
            ShellExecuteW(
                None,
                PCWSTR(operation.as_ptr()),
                PCWSTR(url_wide.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            )
        };

        if result.0 as usize <= 32 {
            return Err(format!("ShellExecuteW failed with code {}", result.0 as usize));
        }
    }
    Ok(())
}

/// Tauri command: unregister old hotkey and register a new one.
/// Unregisters ALL currently registered shortcuts first so only the new one is active.
#[tauri::command]
fn update_hotkey(app_handle: tauri::AppHandle, new_hotkey: String) -> Result<String, String> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    let _ = app_handle.global_shortcut().unregister_all();

    if new_hotkey.is_empty() {
        return Ok("Hotkey cleared".to_string());
    }

    app_handle.global_shortcut().register(new_hotkey.as_str())
        .map_err(|e| format!("Failed to register {}: {}", new_hotkey, e))?;

    Ok(format!("Registered hotkey: {}", new_hotkey))
}

/// Tauri command: ALT+TAB, wait for user click in target window, then type.
#[tauri::command]
fn type_text(app_handle: tauri::AppHandle, text: String, delay_ms: Option<u64>, keyboard_mode: Option<String>, key_press_delay: Option<u64>, target_layout: Option<String>, double_click: Option<bool>, clear_auto_indent: Option<bool>) -> Result<String, String> {
    let typing_delay_ms = delay_ms.unwrap_or(20);
    let delay = Duration::from_millis(typing_delay_ms);
    let mode = keyboard_mode.unwrap_or_else(|| "unicode".to_string());
    let kp_delay = key_press_delay.unwrap_or(5);
    let layout = target_layout.unwrap_or_else(|| "auto".to_string());
    let dbl_click = double_click.unwrap_or(false);
    #[cfg(windows)]
    let enter_opts = EnterOpts::from_typing_delay(clear_auto_indent.unwrap_or(true), typing_delay_ms);
    #[cfg(not(windows))]
    let _ = clear_auto_indent;

    #[cfg(windows)]
    {
        // Step 1: ALT+TAB to switch windows
        alt_tab();
        thread::sleep(Duration::from_millis(300));

        // Step 2: Tell the UI we're waiting for a click
        let status = if dbl_click { "waiting-for-double-click" } else { "waiting-for-click" };
        let _ = app_handle.emit("paste-status", status);

        // Step 3: Wait for the user to click in the target window (30s timeout).
        // When double-click mode is on, wait for the user's own double-click
        // (two clicks) before typing — the clicks pass through, so a real
        // double-click selects the word under the cursor for overwrite.
        let required_clicks = if dbl_click { 2 } else { 1 };
        wait_for_user_click(required_clicks, Duration::from_secs(30))?;

        // Brief delay to let the target app process the click and set focus
        thread::sleep(Duration::from_millis(150));

        // Step 4: Tell the UI we're now typing
        let _ = app_handle.emit("paste-status", "typing");

        // Step 5: Type the text (skip \r in \r\n sequences to avoid double Enter)
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            // Skip \r if followed by \n (Windows line ending → single Enter)
            if c == '\r' && i + 1 < chars.len() && chars[i + 1] == '\n' {
                i += 1;
                continue;
            }
            match mode.as_str() {
                "vkey" if layout == "en-us" => send_vkey_char_enus(c, kp_delay, enter_opts),
                "vkey" => send_vkey_char(c, kp_delay, enter_opts),
                _ => send_unicode_char(c, enter_opts),
            }
            thread::sleep(delay);
            i += 1;
        }
    }

    Ok(format!("Typed {} characters", text.len()))
}

/// Show/focus the main window, creating it if hidden.
fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Read the `alwaysOnTop` setting from the store. Defaults to false.
fn always_on_top_enabled(app: &tauri::AppHandle) -> bool {
    match app.store("settings.json") {
        Ok(store) => store
            .get("alwaysOnTop")
            .and_then(|v: serde_json::Value| v.as_bool())
            .unwrap_or(false),
        Err(_) => false,
    }
}

/// Prepare a freshly-built child window (settings/about) for display:
/// pin it above the main window when always-on-top is enabled, then focus it.
/// A normal window can't be raised above an always-on-top main window, so the
/// child must itself be always-on-top while open to avoid hiding behind main.
fn focus_child_window(win: &tauri::WebviewWindow, always_on_top: bool) {
    if always_on_top {
        let _ = win.set_always_on_top(true);
    }
    let _ = win.set_focus();
}

/// Position a child window adjacent to the main window using physical pixels.
/// "settings" prefers left of main, others prefer right. Falls back to centering
/// when neither side fits. All arithmetic in physical pixels — no DPI conversion.
fn position_child_near_main(app: &tauri::AppHandle, win: &tauri::WebviewWindow) {
    use tauri::PhysicalPosition;

    let Some(main_win) = app.get_webview_window("main") else {
        println!("[pos] no main window");
        return;
    };
    let Ok(mp) = main_win.outer_position() else {
        println!("[pos] main outer_position failed");
        return;
    };
    let Ok(ms) = main_win.outer_size() else {
        println!("[pos] main outer_size failed");
        return;
    };
    let Ok(cs) = win.outer_size() else {
        println!("[pos] child outer_size failed");
        return;
    };
    let Some(monitor) = main_win.current_monitor().ok().flatten() else {
        println!("[pos] no monitor");
        return;
    };

    let mon_pos = monitor.position();
    let mon_size = monitor.size();
    let mon_left = mon_pos.x;
    let mon_top = mon_pos.y;
    let mon_right = mon_left + mon_size.width as i32;
    let mon_bottom = mon_top + mon_size.height as i32;

    let mx = mp.x;
    let my = mp.y;
    let mw = ms.width as i32;
    let cw = cs.width as i32;
    let ch = cs.height as i32;
    let gap = 16;

    let left_x = mx - cw - gap;
    let right_x = mx + mw + gap;
    let fits_left = left_x >= mon_left;
    let fits_right = right_x + cw <= mon_right;

    let prefer_left = win.label() == "settings";

    let x = if prefer_left {
        if fits_left { left_x } else if fits_right { right_x } else {
            mon_left + (mon_size.width as i32 - cw) / 2
        }
    } else if fits_right { right_x } else if fits_left { left_x } else {
        mon_left + (mon_size.width as i32 - cw) / 2
    };

    let y = my.max(mon_top).min(mon_bottom - ch);

    println!("[pos] {} main=({},{} {}x{}) child={}x{} mon=({},{} {}x{}) -> ({}, {})",
        win.label(), mx, my, mw, ms.height, cw, ch,
        mon_left, mon_top, mon_size.width, mon_size.height, x, y);

    let _ = win.set_position(PhysicalPosition::new(x, y));
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_window_state::Builder::default()
            .with_filter(|label| label == "main")
            .build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--start-minimized"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        show_main_window(app);
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![type_text, update_hotkey, get_version, open_url, open_settings, get_system_proxy, check_accessibility_permission, open_accessibility_settings])
        .setup(|app| {
            // ── System tray ──
            let version = app.config().version.clone().unwrap_or_else(|| "unknown".to_string());
            let version_label = format!("Virtual Copy Paste v{}", version);
            let version_i = MenuItem::with_id(app, "version", &version_label, false, None::<&str>)?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let about_i = MenuItem::with_id(app, "about", "About", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&version_i, &sep1, &show_i, &settings_i, &about_i, &separator, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Virtual Copy Paste")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => show_main_window(app),
                        "settings" => {
                            // Open or focus settings window
                            let aot = always_on_top_enabled(app);
                            if let Some(win) = app.get_webview_window("settings") {
                                let _ = win.show();
                                focus_child_window(&win, aot);
                            } else if let Ok(win) = tauri::WebviewWindowBuilder::new(
                                app,
                                "settings",
                                tauri::WebviewUrl::App("settings.html".into()),
                            )
                            .title("Settings — Virtual Copy Paste")
                            .inner_size(480.0, 480.0)
                            .resizable(false)
                            .visible(false)
                            .build()
                            {
                                position_child_near_main(app, &win);
                                let _ = win.show();
                                focus_child_window(&win, aot);
                            }
                        }
                        "about" => {
                            // Open or focus about window
                            let aot = always_on_top_enabled(app);
                            if let Some(win) = app.get_webview_window("about") {
                                let _ = win.show();
                                focus_child_window(&win, aot);
                            } else if let Ok(win) = tauri::WebviewWindowBuilder::new(
                                app,
                                "about",
                                tauri::WebviewUrl::App("about.html".into()),
                            )
                            .title("About — Virtual Copy Paste")
                            .inner_size(340.0, 500.0)
                            .resizable(false)
                            .visible(false)
                            .build()
                            {
                                position_child_near_main(app, &win);
                                let _ = win.show();
                                focus_child_window(&win, aot);
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // ── Close to tray instead of quitting ──
            let main_window = app.get_webview_window("main").unwrap();
            let app_handle = app.handle().clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.hide();
                    }
                }
            });

            // ── Register activate hotkey ──
            // Load saved hotkey from store, fall back to default
            use tauri_plugin_global_shortcut::GlobalShortcutExt;
            let default_hotkey = "Ctrl+Shift+V";
            let hotkey_to_register = {
                match app.store("settings.json") {
                    Ok(store) => {
                        store.get("activateHotkey")
                            .and_then(|v: serde_json::Value| v.as_str().map(|s| s.to_string()))
                            .filter(|s: &String| !s.is_empty())
                            .unwrap_or_else(|| default_hotkey.to_string())
                    }
                    Err(e) => {
                        eprintln!("Failed to load settings store, using default hotkey: {}", e);
                        default_hotkey.to_string()
                    }
                }
            };
            match app.global_shortcut().register(hotkey_to_register.as_str()) {
                Ok(_) => println!("Registered activate hotkey: {}", hotkey_to_register),
                Err(e) => {
                    eprintln!("Failed to register {}: {} — trying default", hotkey_to_register, e);
                    if hotkey_to_register != default_hotkey {
                        match app.global_shortcut().register(default_hotkey) {
                            Ok(_) => println!("Registered fallback hotkey: {}", default_hotkey),
                            Err(e2) => eprintln!("Failed to register fallback: {}", e2),
                        }
                    }
                }
            }

            // ── Show window only if NOT launched minimized ──
            let args: Vec<String> = std::env::args().collect();
            let cli_minimized = args.iter().any(|a| a == "--start-minimized");
            let store_minimized = match app.store("settings.json") {
                Ok(store) => store.get("startMinimized")
                    .and_then(|v: serde_json::Value| v.as_bool())
                    .unwrap_or(false),
                Err(_) => false,
            };
            if !cli_minimized && !store_minimized {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.show();
                    let _ = win.set_focus();
                }
            }

            // ── Check for updates in background (only if enabled) ──
            let auto_check = match app.store("settings.json") {
                Ok(store) => store.get("autoCheckUpdates")
                    .and_then(|v: serde_json::Value| v.as_bool())
                    .unwrap_or(true),
                Err(_) => true,
            };
            if auto_check {
                let handle = app.handle().clone();
                let current_version = app.config().version.clone().unwrap_or_default();
                // Ensure system proxy is available to reqwest via environment variable
                #[cfg(windows)]
                if std::env::var("HTTPS_PROXY").is_err() && std::env::var("https_proxy").is_err() {
                    if let Some(proxy) = get_windows_system_proxy() {
                        std::env::set_var("HTTPS_PROXY", &proxy);
                        println!("Using system proxy for update check: {}", proxy);
                    }
                }
                tauri::async_runtime::spawn(async move {
                    match handle.updater().expect("updater not configured").check().await {
                        Ok(Some(update)) => {
                            if update.version != current_version {
                                println!("Update available: {} (current: {})", update.version, current_version);
                                let _ = handle.emit("update-available", serde_json::json!({
                                    "version": update.version
                                }));
                            } else {
                                println!("App is up to date (v{})", current_version);
                            }
                        }
                        Ok(None) => println!("App is up to date"),
                        Err(e) => {
                            eprintln!("Update check failed: {} — if behind a proxy, set HTTPS_PROXY environment variable", e);
                        }
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Virtual Copy Paste");
}
