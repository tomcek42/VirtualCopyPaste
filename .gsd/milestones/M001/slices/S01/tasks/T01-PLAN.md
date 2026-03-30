---
estimated_steps: 1
estimated_files: 6
skills_used: []
---

# T01: Initialize Tauri v2 project with frontend scaffold

Create the Tauri v2 project structure using cargo create-tauri-app or manual init. Set up package.json with dev dependencies (GSAP via CDN or npm). Configure Tauri window to be compact (~420x550px), always-on-top capable, with a clean title. Create the basic HTML entry point with a text input field styled for the app.

## Inputs

- `Virtual Copy Paste.au3`

## Expected Output

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/src/main.rs`
- `src/index.html`
- `src/styles.css`

## Verification

cd src-tauri && cargo build 2>&1 | tail -5; npm run tauri dev launches the window
