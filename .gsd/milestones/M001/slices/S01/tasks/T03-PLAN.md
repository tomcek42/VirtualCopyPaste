---
estimated_steps: 1
estimated_files: 2
skills_used: []
---

# T03: Wire up Yeti animation JS — eye tracking and mouth reactions

Adapt login-animation.js from the vYetti project. Wire the eye-tracking logic to follow the caret position in our text input. Wire mouth size changes based on text length (small/medium/large). Handle focus/blur events. Replace morphSVG calls with CSS transitions or simple TweenMax transforms since morphSVG requires a paid GSAP plugin. Test that eyes follow text input and mouth reacts.

## Inputs

- `src/index.html`
- `src/styles.css`

## Expected Output

- `src/animation.js`

## Verification

npm run tauri dev — type text in input, Yeti eyes follow cursor position, mouth changes size
