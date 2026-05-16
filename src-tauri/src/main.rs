#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
use tauri::{Manager, Emitter};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::UpdaterExt;

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, KEYEVENTF_SCANCODE,
    VIRTUAL_KEY, VK_MENU, VK_TAB, VK_SHIFT, VK_CONTROL, VK_LMENU, VK_RETURN, VK_HOME,
    VkKeyScanW, MapVirtualKeyW, MAP_VIRTUAL_KEY_TYPE,
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
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(windows)]
static CLICK_DETECTED: AtomicBool = AtomicBool::new(false);

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

/// Low-level mouse hook callback. Sets CLICK_DETECTED when left button goes down.
#[cfg(windows)]
unsafe extern "system" fn mouse_hook_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    // WM_LBUTTONDOWN = 0x0201
    if n_code >= 0 && w_param.0 == 0x0201 {
        CLICK_DETECTED.store(true, Ordering::SeqCst);
    }
    CallNextHookEx(None, n_code, w_param, l_param)
}

/// Wait for the user to left-click anywhere on screen.
/// Installs a low-level mouse hook, pumps messages until a click is detected,
/// then removes the hook. Times out after `timeout` duration.
#[cfg(windows)]
fn wait_for_user_click(timeout: Duration) -> Result<(), String> {
    CLICK_DETECTED.store(false, Ordering::SeqCst);

    let hook = unsafe {
        SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), None, 0)
            .map_err(|e| format!("Failed to install mouse hook: {}", e))?
    };

    let start = std::time::Instant::now();
    let mut msg = unsafe { std::mem::zeroed() };

    while !CLICK_DETECTED.load(Ordering::SeqCst) {
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

/// Simulate pressing the Enter key, then clear any auto-indent on the new line.
/// Sends: Enter → Home → Shift+End → Delete to ensure the line starts clean.
#[cfg(windows)]
fn send_enter() {
    use windows::Win32::UI::Input::KeyboardAndMouse::{VK_END, VK_DELETE, KEYEVENTF_EXTENDEDKEY};

    let scan_ret = unsafe { MapVirtualKeyW(VK_RETURN.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let scan_home = unsafe { MapVirtualKeyW(VK_HOME.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let scan_end = unsafe { MapVirtualKeyW(VK_END.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let scan_del = unsafe { MapVirtualKeyW(VK_DELETE.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;
    let scan_shift = unsafe { MapVirtualKeyW(VK_SHIFT.0 as u32, MAP_VIRTUAL_KEY_TYPE(0)) } as u16;

    let ext = KEYEVENTF_EXTENDEDKEY;
    let ext_up = KEYBD_EVENT_FLAGS(KEYEVENTF_EXTENDEDKEY.0 | KEYEVENTF_KEYUP.0);

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

    // Brief pause for the target app to process Enter and insert auto-indent
    thread::sleep(Duration::from_millis(30));

    // Home → Shift+End → Delete (select all auto-indent on new line and remove it)
    // Home, End, Delete need KEYEVENTF_EXTENDEDKEY to avoid being interpreted as Numpad keys
    let mut clear_inputs: [INPUT; 8] = [
        // Home down (extended)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_HOME, wScan: scan_home, dwFlags: ext, time: 0, dwExtraInfo: 0 } } },
        // Home up (extended)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_HOME, wScan: scan_home, dwFlags: ext_up, time: 0, dwExtraInfo: 0 } } },
        // Shift down
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_SHIFT, wScan: scan_shift, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 } } },
        // End down (extended, with Shift held = select to end)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_END, wScan: scan_end, dwFlags: ext, time: 0, dwExtraInfo: 0 } } },
        // End up (extended)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_END, wScan: scan_end, dwFlags: ext_up, time: 0, dwExtraInfo: 0 } } },
        // Shift up
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_SHIFT, wScan: scan_shift, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 } } },
        // Delete down (extended)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_DELETE, wScan: scan_del, dwFlags: ext, time: 0, dwExtraInfo: 0 } } },
        // Delete up (extended)
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_DELETE, wScan: scan_del, dwFlags: ext_up, time: 0, dwExtraInfo: 0 } } },
    ];
    unsafe { SendInput(&mut clear_inputs, std::mem::size_of::<INPUT>() as i32); }
}

/// Send a single Unicode character as a keypress via Windows SendInput.
#[cfg(windows)]
fn send_unicode_char(c: char) {
    // Newlines → send Enter keypress instead of Unicode control character
    if c == '\n' || c == '\r' {
        send_enter();
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

/// Send a character by simulating real key presses via VkKeyScanW (compatible with VDI/Remote).
/// Maps the character to a virtual key code + modifier state, then sends scancode-based input.
/// Sends modifier and key events separately with small delays between them to ensure
/// nested remote sessions (RDP → VDI → Console) process each event correctly.
/// Falls back to Unicode mode for characters not in the current keyboard layout.
#[cfg(windows)]
fn send_vkey_char(c: char, key_delay_ms: u64) {
    // Newlines → send Enter keypress
    if c == '\n' || c == '\r' {
        send_enter();
        return;
    }

    // Encode to UTF-16 and use VkKeyScanW to find the virtual key
    let mut buf = [0u16; 2];
    let encoded = c.encode_utf16(&mut buf);

    // VkKeyScanW only works with BMP characters (single u16)
    if encoded.len() != 1 {
        // Supplementary character — fall back to unicode mode
        send_unicode_char(c);
        return;
    }

    let result = unsafe { VkKeyScanW(encoded[0]) };

    // VkKeyScanW returns -1 if the character can't be mapped
    if result == -1 {
        // Character not available in current keyboard layout — fall back to unicode
        send_unicode_char(c);
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

/// Tauri command: open or focus the settings window.
/// Uses run_on_main_thread to avoid deadlocking when called from a webview invoke.
#[tauri::command]
async fn open_settings(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app_handle.get_webview_window("settings") {
        let _ = win.show();
        let _ = win.set_focus();
        return Ok(());
    }
    let handle = app_handle.clone();
    app_handle.run_on_main_thread(move || {
        let _ = tauri::WebviewWindowBuilder::new(
            &handle,
            "settings",
            tauri::WebviewUrl::App("settings.html".into()),
        )
        .title("Settings — Virtual Copy Paste")
        .inner_size(480.0, 450.0)
        .resizable(true)
        .center()
        .build();
    }).map_err(|e| format!("Failed to open settings: {}", e))?;
    Ok(())
}

/// Tauri command: return the app version from tauri.conf.json.
#[tauri::command]
fn get_version(app_handle: tauri::AppHandle) -> String {
    app_handle.config().version.clone().unwrap_or_else(|| "unknown".to_string())
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
    #[cfg(not(windows))]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }
    Ok(())
}

/// Tauri command: unregister old hotkey and register a new one.
#[tauri::command]
fn update_hotkey(app_handle: tauri::AppHandle, old_hotkey: Option<String>, new_hotkey: String) -> Result<String, String> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    // Unregister old hotkey if provided
    if let Some(ref old) = old_hotkey {
        if !old.is_empty() {
            let _ = app_handle.global_shortcut().unregister(old.as_str());
        }
    }

    // Register new hotkey
    if new_hotkey.is_empty() {
        return Ok("Hotkey cleared".to_string());
    }

    app_handle.global_shortcut().register(new_hotkey.as_str())
        .map_err(|e| format!("Failed to register {}: {}", new_hotkey, e))?;

    Ok(format!("Registered hotkey: {}", new_hotkey))
}

/// Tauri command: ALT+TAB, wait for user click in target window, then type.
#[tauri::command]
fn type_text(app_handle: tauri::AppHandle, text: String, delay_ms: Option<u64>, keyboard_mode: Option<String>, key_press_delay: Option<u64>) -> Result<String, String> {
    let delay = Duration::from_millis(delay_ms.unwrap_or(20));
    let mode = keyboard_mode.unwrap_or_else(|| "unicode".to_string());
    let kp_delay = key_press_delay.unwrap_or(5);

    #[cfg(windows)]
    {
        // Step 1: ALT+TAB to switch windows
        alt_tab();
        thread::sleep(Duration::from_millis(300));

        // Step 2: Tell the UI we're waiting for a click
        let _ = app_handle.emit("paste-status", "waiting-for-click");

        // Step 3: Wait for the user to click in the target window (30s timeout)
        wait_for_user_click(Duration::from_secs(30))?;

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
                "vkey" => send_vkey_char(c, kp_delay),
                _ => send_unicode_char(c),
            }
            thread::sleep(delay);
            i += 1;
        }
    }

    #[cfg(not(windows))]
    return Err("Keyboard simulation is only supported on Windows".to_string());

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

fn main() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![type_text, update_hotkey, get_version, open_url, open_settings])
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
                            if let Some(win) = app.get_webview_window("settings") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            } else {
                                let _ = tauri::WebviewWindowBuilder::new(
                                    app,
                                    "settings",
                                    tauri::WebviewUrl::App("settings.html".into()),
                                )
                                .title("Settings — Virtual Copy Paste")
                                .inner_size(480.0, 450.0)
                                .resizable(false)
                                .center()
                                .build();
                            }
                        }
                        "about" => {
                            // Open or focus about window
                            if let Some(win) = app.get_webview_window("about") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            } else {
                                let _ = tauri::WebviewWindowBuilder::new(
                                    app,
                                    "about",
                                    tauri::WebviewUrl::App("about.html".into()),
                                )
                                .title("About — Virtual Copy Paste")
                                .inner_size(340.0, 500.0)
                                .resizable(false)
                                .center()
                                .build();
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
                tauri::async_runtime::spawn(async move {
                    match handle.updater().expect("updater not configured").check().await {
                        Ok(Some(update)) => {
                            println!("Update available: {}", update.version);
                            let _ = handle.emit("update-available", serde_json::json!({
                                "version": update.version
                            }));
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
