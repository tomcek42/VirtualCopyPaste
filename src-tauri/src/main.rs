#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
use tauri::{Manager, Emitter};

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, MOUSEINPUT,
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    VIRTUAL_KEY, VK_MENU, VK_TAB,
};

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos};

#[cfg(windows)]
use windows::Win32::Foundation::POINT;

/// Simulate ALT+TAB to switch to the next window.
#[cfg(windows)]
fn alt_tab() {
    let mut inputs: [INPUT; 4] = [
        // ALT down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_MENU, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        // TAB down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_TAB, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 },
            },
        },
        // TAB up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_TAB, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
        // ALT up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT { wVk: VK_MENU, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
            },
        },
    ];
    unsafe { SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32); }
}

/// Get the current cursor position.
#[cfg(windows)]
fn get_cursor_pos() -> Result<POINT, String> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point)
            .map_err(|e| format!("GetCursorPos failed: {}", e))?;
    }
    Ok(point)
}

/// Move the cursor to a specific screen position and click.
#[cfg(windows)]
fn click_at(x: i32, y: i32) {
    // Move cursor to saved position
    unsafe { let _ = SetCursorPos(x, y); }

    // Small delay to let the cursor settle
    thread::sleep(Duration::from_millis(50));

    // Send left mouse button down + up
    let mut inputs: [INPUT; 2] = [
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    unsafe { SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32); }
}

/// Send a single Unicode character as a keypress via Windows SendInput.
#[cfg(windows)]
fn send_unicode_char(c: char) {
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

/// Tauri command: save cursor position, ALT+TAB, click at saved position, then type.
#[tauri::command]
fn type_text(text: String, delay_ms: Option<u64>) -> Result<String, String> {
    let delay = Duration::from_millis(delay_ms.unwrap_or(20));

    #[cfg(windows)]
    {
        // 1. Save current cursor position (where user left it in the target app)
        let saved_pos = get_cursor_pos()?;

        // 2. Switch to previous window via ALT+TAB
        alt_tab();

        // 3. Wait for window switch animation to complete
        thread::sleep(Duration::from_millis(500));

        // 4. Click at saved cursor position to ensure input focus
        click_at(saved_pos.x, saved_pos.y);

        // 5. Small delay for focus to settle
        thread::sleep(Duration::from_millis(100));

        // 6. Type each character
        for c in text.chars() {
            send_unicode_char(c);
            thread::sleep(delay);
        }
    }

    #[cfg(not(windows))]
    return Err("Keyboard simulation is only supported on Windows".to_string());

    Ok(format!("Typed {} characters", text.len()))
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("hotkey-paste", 0u32);
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![type_text])
        .setup(|app| {
            use tauri_plugin_global_shortcut::GlobalShortcutExt;
            match app.global_shortcut().register("Ctrl+Shift+V") {
                Ok(_) => println!("Registered global shortcut: Ctrl+Shift+V"),
                Err(e) => eprintln!("Failed to register Ctrl+Shift+V: {}", e),
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Virtual Copy Paste");
}
