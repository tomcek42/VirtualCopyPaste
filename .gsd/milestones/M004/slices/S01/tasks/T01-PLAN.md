---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Switch get_store to store at setup-time hotkey read

In src-tauri/src/main.rs, replace app.get_store("settings.json") with app.store("settings.json") inside the setup() closure that reads activateHotkey for initial registration. Update the match arm from Some/None to Ok/Err. Log the Err path via eprintln so a disk-load failure is visible rather than silently falling back to the default hotkey.

## Inputs

- `Current main.rs with app.get_store call at the hotkey-registration site.`
- `tauri-plugin-store 2.x docs distinguishing get_store (cache-only) from store (load-on-miss).`

## Expected Output

- `main.rs uses app.store with Ok/Err match.`
- `Err arm logs via eprintln.`
- `Saved hotkey survives quit+relaunch, confirmed by console log line and live trigger.`

## Verification

User runs npm run tauri:dev, sets a non-default hotkey (e.g. F6) in Settings, saves, quits via tray → Quit, relaunches. Console log line must read 'Registered activate hotkey: F6' (or whatever was saved), not the default. Pressing the saved hotkey must trigger the overlay.
