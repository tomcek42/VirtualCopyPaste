---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T02: Port Yeti SVG and CSS into the frontend

Extract the Yeti SVG markup from the vYetti project's unpentry.jsp. Clean it up — remove vSphere-specific elements, keep the animated SVG with all named classes (eyeL, eyeR, armL, armR, mouth, face, etc.). Port the app.component.css styles, adapting colors/sizing for our compact dark-themed UI. Include GSAP via CDN (TweenMax). Ensure the Yeti renders centered above the text input.

## Inputs

- `src/index.html`
- `src/styles.css`

## Expected Output

- `src/index.html`
- `src/styles.css`

## Verification

npm run tauri dev — Yeti SVG renders visually in the window above the input field
