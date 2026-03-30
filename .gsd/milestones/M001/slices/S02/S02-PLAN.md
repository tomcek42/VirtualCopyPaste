# S02: Core Paste Functionality (Character-by-Character)

**Goal:** Implement character-by-character paste into target windows via Rust backend keyboard simulation.
**Demo:** After this: Enter text, click paste, text appears character-by-character in another window after 1.5s delay.

## Tasks
- [x] **T01: Rust type_text command sends characters via Windows SendInput with Unicode support.** — Add a Tauri command `type_text` in main.rs that takes a string and a delay_ms parameter. Use the Windows SendInput API (via winapi crate or raw FFI) to simulate keyboard input character by character. Each character is sent as a Unicode keypress event (KEYBDINPUT with KEYEVENTF_UNICODE). Sleep for delay_ms between each character. Add a 1.5s initial delay before typing starts (so user can switch to target window).
  - Estimate: 30m
  - Files: src-tauri/src/main.rs, src-tauri/Cargo.toml
  - Verify: cargo build succeeds. Invoke type_text from frontend, text appears in Notepad.
- [x] **T02: Frontend paste button wired to Tauri command with countdown and status feedback.** — Wire the Paste button in index.html to call the Tauri type_text command. Use the Tauri JS API (window.__TAURI__.core.invoke) to call the Rust backend. Show a countdown or status indicator while the initial delay runs. Disable the button during typing. Re-enable when done.
  - Estimate: 20m
  - Files: src/index.html, src/app.js
  - Verify: Click paste button, switch to Notepad within 1.5s, text appears character-by-character.
