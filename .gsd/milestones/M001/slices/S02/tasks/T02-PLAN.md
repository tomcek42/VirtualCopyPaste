---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: Wire frontend paste button to Tauri command with status feedback

Wire the Paste button in index.html to call the Tauri type_text command. Use the Tauri JS API (window.__TAURI__.core.invoke) to call the Rust backend. Show a countdown or status indicator while the initial delay runs. Disable the button during typing. Re-enable when done.

## Inputs

- `src/index.html`
- `src/animation.js`

## Expected Output

- `src/index.html`
- `src/app.js`

## Verification

Click paste button, switch to Notepad within 1.5s, text appears character-by-character.
