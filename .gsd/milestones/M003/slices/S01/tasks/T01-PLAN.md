---
estimated_steps: 6
estimated_files: 3
skills_used: []
---

# T01: Replace emoji icons with SVG graphics in UI

1. Replace emoji text in maskToggle button with inline SVG img elements
2. In app.js handleMaskToggle(): swap between eye.svg (unmask state) and eye-off.svg (masked state)
3. Update initial button content to show eye.svg
4. Style the SVG img to fit the button (size, color, alignment)
5. Replace emoji in pasteBtn with clean text or SVG icon
6. Update styles.css for the new icon sizing

## Inputs

- `eye.svg`
- `eye-off.svg`
- `src/index.html`
- `src/app.js`
- `src/styles.css`

## Expected Output

- `src/index.html (SVG img in maskToggle button)`
- `src/app.js (toggle between eye.svg and eye-off.svg)`
- `src/styles.css (icon sizing)`

## Verification

npx tauri dev — verify mask toggle shows SVG eye icons, paste button has no emoji
