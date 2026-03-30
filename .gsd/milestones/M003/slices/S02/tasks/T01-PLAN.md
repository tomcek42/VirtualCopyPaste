---
estimated_steps: 5
estimated_files: 2
skills_used: []
---

# T01: Implement save-cursor-position and click-to-focus in Rust backend

1. Add required Windows API features to Cargo.toml: Win32_UI_WindowsAndMessaging (GetCursorPos, SetCursorPos), Win32_Foundation (POINT)
2. In type_text command: call GetCursorPos before alt_tab to save position
3. After alt_tab + wait: call SetCursorPos to restore, then send mouse click (left down + left up) at that position
4. Then proceed with character-by-character typing
5. Test with npx tauri dev against Notepad

## Inputs

- `src-tauri/src/main.rs`
- `src-tauri/Cargo.toml`

## Expected Output

- `src-tauri/src/main.rs (smart focus paste)`
- `src-tauri/Cargo.toml (additional Windows API features if needed)`

## Verification

npx tauri dev — type text, click Paste, verify text appears in Notepad/CMD at cursor position
