---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T02: Updated tauri.conf.json with macOS icon paths and dmg/app bundle targets

Add .icns icon path to the icon array. Add dmg to bundle targets so macOS builds produce a .dmg. Keep nsis for Windows. Add macOS-specific bundle section if needed.

## Inputs

- `src-tauri/tauri.conf.json`

## Expected Output

- `src-tauri/tauri.conf.json`

## Verification

JSON is valid, icon paths exist, targets include both nsis and dmg
