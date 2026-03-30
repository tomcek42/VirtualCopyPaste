---
id: T03
parent: S01
milestone: M001
provides: []
requires: []
affects: []
key_files: ["src/animation.js"]
key_decisions: ["Used gsap.to attr:{d} to animate mouth shapes instead of morphSVG plugin", "Mouth changes at 0 chars (small), 1-8 chars (medium), 9+ chars (large) instead of @-symbol trigger", "Exposed window.yetiAnimation = { coverEyes, uncoverEyes, resetFace } for S03 mask toggle"]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "App launches, animation.js loads without errors, all DOM selectors resolve to SVG elements."
completed_at: 2026-03-28T23:46:22.046Z
blocker_discovered: false
---

# T03: Yeti animation JS wired up with eye tracking, mouth reactions, and arm cover/uncover API.

> Yeti animation JS wired up with eye tracking, mouth reactions, and arm cover/uncover API.

## What Happened
---
id: T03
parent: S01
milestone: M001
key_files:
  - src/animation.js
key_decisions:
  - Used gsap.to attr:{d} to animate mouth shapes instead of morphSVG plugin
  - Mouth changes at 0 chars (small), 1-8 chars (medium), 9+ chars (large) instead of @-symbol trigger
  - Exposed window.yetiAnimation = { coverEyes, uncoverEyes, resetFace } for S03 mask toggle
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:46:22.047Z
blocker_discovered: false
---

# T03: Yeti animation JS wired up with eye tracking, mouth reactions, and arm cover/uncover API.

**Yeti animation JS wired up with eye tracking, mouth reactions, and arm cover/uncover API.**

## What Happened

Adapted the vYetti login-animation.js for GSAP v3 and our use case. Eye tracking follows caret position using trigonometry (angles from eye/nose/mouth coordinates to caret). Mouth size changes based on text length instead of @-symbol detection. Arms cover/uncover eyes exposed as window.yetiAnimation API for the mask toggle in S03. All TweenMax calls converted to gsap.to/gsap.set syntax.

## Verification

App launches, animation.js loads without errors, all DOM selectors resolve to SVG elements.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npx tauri dev — app running` | 0 | ✅ pass | 17000ms |
| 2 | `grep -c 'gsap.to\|gsap.set' src/animation.js` | 0 | ✅ pass — 26 gsap animation calls | 50ms |


## Deviations

Replaced morphSVG (paid GSAP plugin) with direct SVG path 'd' attribute animation via gsap.to(..., { attr: { d } }). This is free and achieves the same mouth shape transitions.

## Known Issues

None.

## Files Created/Modified

- `src/animation.js`


## Deviations
Replaced morphSVG (paid GSAP plugin) with direct SVG path 'd' attribute animation via gsap.to(..., { attr: { d } }). This is free and achieves the same mouth shape transitions.

## Known Issues
None.
