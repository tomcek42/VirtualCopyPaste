---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T01: Implement Rust type_text command with Windows SendInput

Add a Tauri command `type_text` in main.rs that takes a string and a delay_ms parameter. Use the Windows SendInput API (via winapi crate or raw FFI) to simulate keyboard input character by character. Each character is sent as a Unicode keypress event (KEYBDINPUT with KEYEVENTF_UNICODE). Sleep for delay_ms between each character. Add a 1.5s initial delay before typing starts (so user can switch to target window).

## Inputs

- `src-tauri/src/main.rs`

## Expected Output

- `src-tauri/src/main.rs`
- `src-tauri/Cargo.toml`

## Verification

cargo build succeeds. Invoke type_text from frontend, text appears in Notepad.
