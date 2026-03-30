# S02: Smart Focus Paste — Click at Saved Cursor Position

**Goal:** Implement reliable window switch with focus click at the last known cursor position before the switch.
**Demo:** After this: After this, pressing Paste switches to the previous window, clicks where the user's mouse was, and types the text there.

## Tasks
- [x] **T01: Implemented save-cursor → Alt+Tab → click-at-position → type flow in Rust backend.** — 1. Add required Windows API features to Cargo.toml: Win32_UI_WindowsAndMessaging (GetCursorPos, SetCursorPos), Win32_Foundation (POINT)
2. In type_text command: call GetCursorPos before alt_tab to save position
3. After alt_tab + wait: call SetCursorPos to restore, then send mouse click (left down + left up) at that position
4. Then proceed with character-by-character typing
5. Test with npx tauri dev against Notepad
  - Estimate: 20min
  - Files: src-tauri/src/main.rs, src-tauri/Cargo.toml
  - Verify: npx tauri dev — type text, click Paste, verify text appears in Notepad/CMD at cursor position
