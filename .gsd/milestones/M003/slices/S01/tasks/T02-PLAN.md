---
estimated_steps: 3
estimated_files: 1
skills_used: []
---

# T02: Convert textbox.svg to app icon

1. Convert textbox.svg to multi-resolution icon.ico (16, 32, 48, 256px) using a tool or manual process
2. Replace src-tauri/icons/icon.ico with the new icon
3. Verify Tauri picks up the new icon in dev mode

## Inputs

- `textbox.svg`

## Expected Output

- `src-tauri/icons/icon.ico (new icon from textbox.svg)`

## Verification

npx tauri dev — verify app icon in taskbar and title bar shows textbox icon
