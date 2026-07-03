---
id: S01
parent: M004
milestone: M004
provides:
  - ["Persisted activateHotkey is honored on cold start.", "Observable log line on store-load failure."]
requires:
  []
affects:
  []
key_files:
  - ["src-tauri/src/main.rs"]
key_decisions:
  - ["Use app.store rather than app.get_store at setup — the former loads from disk, the latter only hits the in-memory cache that is empty until a webview calls store.load.", "Log the Err path via eprintln so a real disk-load failure is visible rather than masquerading as a missing hotkey."]
patterns_established:
  - (none)
observability_surfaces:
  - none
drill_down_paths:
  []
duration: ""
verification_result: passed
completed_at: 2026-04-24T18:10:21.607Z
blocker_discovered: false
---

# S01: Hotkey persistence fix

**Load the settings store from disk at startup so the persisted hotkey wins over the default.**

## What Happened

Root cause: main.rs called app.get_store at setup time. In tauri-plugin-store 2.x, get_store is a cache-only lookup that returns None when no webview has yet called store.load. At startup the settings webview has not run, so the lookup always returned None and the code fell through to the default Ctrl+Shift+Space regardless of what the user had saved. Fix: switch to app.store(path), which internally builds and loads the store from disk on first access. Match arm updated from Some/None to Ok/Err; the Err path now logs via eprintln so a real disk-load failure is observable rather than silently masquerading as a missing hotkey. Confirmed against tauri-plugin-store source: store() builds and loads, get_store() only checks the in-memory StoreState. User verified: set F6, saved, quit via tray, relaunched, console emitted "Registered activate hotkey: F6" (previously the default was always registered).

## Verification

User-confirmed reproduction and fix via npm run tauri:dev. Before fix the startup log always showed "Registered activate hotkey: Ctrl+Shift+Space" regardless of the saved value. After fix the log shows "Registered activate hotkey: F6" and F6 triggers the overlay.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

None.

## Deviations

None.

## Known Limitations

None.

## Follow-ups

S02 (resume/wake + runtime health-check) is next — the hotkey persists now, but still may not survive a suspend/resume cycle.

## Files Created/Modified

- `src-tauri/src/main.rs` — Replaced get_store (cache-only, returns None before webview loads) with store (loads from disk). Match arm updated from Some/None to Ok/Err; Err path now logs via eprintln instead of silently defaulting.
