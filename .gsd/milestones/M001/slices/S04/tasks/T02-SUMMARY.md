---
id: T02
parent: S04
milestone: M001
provides: []
requires: []
affects: []
key_files: ["src-tauri/src/main.rs", "src-tauri/Cargo.toml", "src-tauri/capabilities/default.json", "src/app.js"]
key_decisions: ["Registered shortcuts Ctrl+Shift+1/2/3 in Rust setup() hook", "Emit 'hotkey-paste' event to frontend with slot index", "Frontend listens via window.__TAURI__.event.listen()", "Used tauri-plugin-global-shortcut v2.3.1"]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "All 3 global shortcuts registered on app launch. Build clean."
completed_at: 2026-03-28T23:57:18.202Z
blocker_discovered: false
---

# T02: Global hotkeys Ctrl+Shift+1/2/3 registered and wired to slot paste via Tauri events.

> Global hotkeys Ctrl+Shift+1/2/3 registered and wired to slot paste via Tauri events.

## What Happened
---
id: T02
parent: S04
milestone: M001
key_files:
  - src-tauri/src/main.rs
  - src-tauri/Cargo.toml
  - src-tauri/capabilities/default.json
  - src/app.js
key_decisions:
  - Registered shortcuts Ctrl+Shift+1/2/3 in Rust setup() hook
  - Emit 'hotkey-paste' event to frontend with slot index
  - Frontend listens via window.__TAURI__.event.listen()
  - Used tauri-plugin-global-shortcut v2.3.1
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:57:18.203Z
blocker_discovered: false
---

# T02: Global hotkeys Ctrl+Shift+1/2/3 registered and wired to slot paste via Tauri events.

**Global hotkeys Ctrl+Shift+1/2/3 registered and wired to slot paste via Tauri events.**

## What Happened

Added tauri-plugin-global-shortcut via 'npx tauri add global-shortcut'. Registered Ctrl+Shift+1/2/3 in the Rust setup() hook. Each shortcut emits a 'hotkey-paste' event to the frontend with the slot index. Frontend listens for the event and calls pasteSlot(). All 3 shortcuts registered successfully on launch. Added capabilities/default.json with global-shortcut permissions.

## Verification

All 3 global shortcuts registered on app launch. Build clean.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd src-tauri && cargo build` | 0 | ✅ pass — clean build | 32680ms |
| 2 | `npx tauri dev — stdout shows 3 registered shortcuts` | 0 | ✅ pass — Registered global shortcut: Ctrl+Shift+1/2/3 | 22000ms |


## Deviations

Used tauri-plugin-global-shortcut with Rust-side registration and event emission instead of JS-side API (more reliable for static HTML apps).

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
- `src-tauri/Cargo.toml`
- `src-tauri/capabilities/default.json`
- `src/app.js`


## Deviations
Used tauri-plugin-global-shortcut with Rust-side registration and event emission instead of JS-side API (more reliable for static HTML apps).

## Known Issues
None.
