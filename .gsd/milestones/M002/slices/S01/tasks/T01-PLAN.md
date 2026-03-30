---
estimated_steps: 9
estimated_files: 2
skills_used: []
---

# T01: Remove mouth morph logic from animation.js

1. Read animation.js fully
2. Remove all morphSVG calls (mouthBG, mouthOutline, mouthMaskPath transitions)
3. Remove mouthStatus variable and all mouth state tracking
4. Remove tooth and tongue animation tweens
5. Remove eye scale changes tied to mouth state (scaleX/scaleY changes in onEmailInput)
6. Keep: getCoord, eye-tracking gsap.to calls, nose/face/chin/ear/hair tracking, coverEyes, uncoverEyes, resetFace
7. Remove variables for mouthBG, mouthSmallBG, mouthMediumBG, mouthLargeBG, mouthMaskPath, mouthOutline, tooth, tongue
8. Remove the MorphSVGPlugin.min.js script tag and gsap.registerPlugin(MorphSVGPlugin) from index.html
9. Delete src/MorphSVGPlugin.min.js file

## Inputs

- `src/animation.js`
- `src/index.html`

## Expected Output

- `src/animation.js (cleaned)`
- `src/index.html (MorphSVGPlugin script tag removed)`
- `src/MorphSVGPlugin.min.js (deleted)`

## Verification

npx tauri dev — verify: eyes track text input, arms cover on mask toggle, mouth stays closed, no JS console errors
