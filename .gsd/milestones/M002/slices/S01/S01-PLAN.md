# S01: Fix Yeti Animation — Remove Mouth Morph & MorphSVGPlugin

**Goal:** Remove all mouth morph logic and MorphSVGPlugin dependency while preserving eye-tracking and arm-cover animations.
**Demo:** After this: After this, the Yeti has a permanently closed mouth, eyes track text, arms cover on mask — MorphSVGPlugin is gone.

## Tasks
- [x] **T01: Removed mouth morph logic and MorphSVGPlugin — Yeti mouth stays closed, eyes still track text.** — 1. Read animation.js fully
2. Remove all morphSVG calls (mouthBG, mouthOutline, mouthMaskPath transitions)
3. Remove mouthStatus variable and all mouth state tracking
4. Remove tooth and tongue animation tweens
5. Remove eye scale changes tied to mouth state (scaleX/scaleY changes in onEmailInput)
6. Keep: getCoord, eye-tracking gsap.to calls, nose/face/chin/ear/hair tracking, coverEyes, uncoverEyes, resetFace
7. Remove variables for mouthBG, mouthSmallBG, mouthMediumBG, mouthLargeBG, mouthMaskPath, mouthOutline, tooth, tongue
8. Remove the MorphSVGPlugin.min.js script tag and gsap.registerPlugin(MorphSVGPlugin) from index.html
9. Delete src/MorphSVGPlugin.min.js file
  - Estimate: 15min
  - Files: src/animation.js, src/index.html
  - Verify: npx tauri dev — verify: eyes track text input, arms cover on mask toggle, mouth stays closed, no JS console errors
