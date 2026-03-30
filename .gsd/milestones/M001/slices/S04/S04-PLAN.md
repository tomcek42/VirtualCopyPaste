# S04: Multi-Slot Clipboard + Global Hotkeys

**Goal:** Add multiple clipboard slots and global hotkey support for triggering paste from any application.
**Demo:** After this: Store text in 3 slots, switch between them. Use global hotkey to trigger paste from any app.

## Tasks
- [x] **T01: Multi-slot clipboard UI with 3 numbered slots, auto-save, and content indicators.** — Add a slot selector UI with 3 numbered buttons (1, 2, 3) above the text input. Each slot stores independent text. Active slot is highlighted. Switching slots updates the text input to show that slot's content. Text input changes save to the active slot automatically. Style slots as compact numbered buttons in a row.
  - Estimate: 20m
  - Files: src/index.html, src/styles.css, src/app.js
  - Verify: Click between slots — each stores independent text. Active slot highlighted.
- [x] **T02: Global hotkeys Ctrl+Shift+1/2/3 registered and wired to slot paste via Tauri events.** — Register global hotkeys (Ctrl+Shift+1, Ctrl+Shift+2, Ctrl+Shift+3) in the Rust backend using the Tauri global-shortcut plugin or raw Windows API. Each hotkey selects the corresponding slot and triggers the type_text command with that slot's content. Emit events to the frontend to update the active slot indicator when a hotkey is pressed.
  - Estimate: 30m
  - Files: src-tauri/src/main.rs, src-tauri/Cargo.toml, src-tauri/tauri.conf.json, src/app.js
  - Verify: Ctrl+Shift+1 from any app triggers paste of slot 1 content.
