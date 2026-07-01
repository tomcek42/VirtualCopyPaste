---
id: T01
parent: S01
milestone: M004
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - Use app.store rather than app.get_store at setup — the former loads from disk, the latter only hits an empty in-memory cache at that lifecycle point.
  - Log the Err arm via eprintln so disk-load failures are observable.
duration: 
verification_result: passed
completed_at: 2026-04-24T18:10:02.239Z
blocker_discovered: false
---

# T01: Switched app.get_store to app.store at setup so the persisted hotkey loads from disk on cold start.

**Switched app.get_store to app.store at setup so the persisted hotkey loads from disk on cold start.**

## What Happened

Replaced the cache-only app.get_store lookup with app.store, which builds the store from disk on first access. Updated the match arm from Some/None to Ok/Err. The Err arm now logs the underlying error via eprintln so a real disk-load failure is visible instead of being masked as a missing hotkey. Net diff is about five lines in src-tauri/src/main.rs at the setup-time hotkey-registration site.

## Verification

User ran the fix in npm run tauri:dev. After save+quit+relaunch the console now emits "Registered activate hotkey: F6" matching the saved value, and pressing F6 triggers the overlay. Before the fix, the same sequence always printed "Registered activate hotkey: Ctrl+Shift+Space".

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `manual: set F6 hotkey, save, Tray Quit, relaunch via npm run tauri:dev, read startup log line` | 0 | pass | 0ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
