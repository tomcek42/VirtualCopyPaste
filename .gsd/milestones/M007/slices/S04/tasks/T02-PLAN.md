---
estimated_steps: 1
estimated_files: 4
skills_used: []
---

# T02: Added startup accessibility check, frontend banner with Grant Access button, and auto-dismiss on focus when permission granted

In the Tauri setup hook (macOS only), check AXIsProcessTrusted and emit an accessibility-missing event if false. In the frontend, listen for this event and show a non-blocking banner (similar to the update notice) with text explaining the permission and a button to open System Settings. Add corresponding HTML element and CSS styling.

## Inputs

- `src/app.js (update notice pattern)`
- `src/index.html (existing banner markup)`
- `src/styles.css (existing banner styles)`

## Expected Output

- `src-tauri/src/main.rs`
- `src/app.js`
- `src/index.html`
- `src/styles.css`

## Verification

cargo check passes, HTML/CSS/JS consistent, banner element exists in index.html
