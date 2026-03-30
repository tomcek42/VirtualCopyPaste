---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T01: Add mask toggle button with Yeti eye-covering animation

Add a mask toggle button (eye icon) next to the text input. Wire it to: 1) toggle input type between 'text' and 'password', 2) call window.yetiAnimation.coverEyes() or uncoverEyes(), 3) update the button icon (👁️ / 🙈). Style the toggle button as a compact icon button overlaid on the right side of the input field.

## Inputs

- `src/index.html`
- `src/styles.css`
- `src/app.js`
- `src/animation.js`

## Expected Output

- `src/index.html`
- `src/styles.css`
- `src/app.js`

## Verification

Click mask button — input becomes password field, Yeti covers eyes. Click again — unmask, eyes uncover.
