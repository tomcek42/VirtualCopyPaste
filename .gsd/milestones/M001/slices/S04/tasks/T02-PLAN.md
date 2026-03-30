---
estimated_steps: 1
estimated_files: 4
skills_used: []
---

# T02: Register global hotkeys for slot paste via Tauri plugin

Register global hotkeys (Ctrl+Shift+1, Ctrl+Shift+2, Ctrl+Shift+3) in the Rust backend using the Tauri global-shortcut plugin or raw Windows API. Each hotkey selects the corresponding slot and triggers the type_text command with that slot's content. Emit events to the frontend to update the active slot indicator when a hotkey is pressed.

## Inputs

- `src-tauri/src/main.rs`
- `src/app.js`

## Expected Output

- `src-tauri/src/main.rs`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src/app.js`

## Verification

Ctrl+Shift+1 from any app triggers paste of slot 1 content.
