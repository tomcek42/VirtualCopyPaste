# S01: Hotkey persistence fix — UAT

**Milestone:** M004
**Written:** 2026-04-24T18:10:21.607Z

## UAT — S01 Hotkey persistence fix

### Scenario 1: Persisted hotkey survives a full restart
Given a fresh launch of the app, when the user opens Settings, records a new hotkey (e.g. F6), presses Save, quits via tray, and relaunches, then the console logs `Registered activate hotkey: F6` and pressing F6 triggers the overlay, while the previous default `Ctrl+Shift+Space` no longer triggers it.

Status: PASS (user-confirmed, log line `Registered activate hotkey: F6` after relaunch).

### Scenario 2: Missing / unreadable store falls back cleanly
Given a corrupt or unreadable `settings.json`, when the app starts, then the console logs `Failed to load settings store, using default hotkey: <reason>` and the default `Ctrl+Shift+Space` is registered so the app remains usable.

Status: Not explicitly tested; fallback path exists and is logged.
