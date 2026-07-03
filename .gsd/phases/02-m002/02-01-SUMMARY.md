---
id: S01
parent: M002
milestone: M002
provides:
  - Clean animation.js without MorphSVGPlugin dependency
requires:
  []
affects:
  - S02
key_files:
  - src/animation.js
  - src/index.html
key_decisions:
  - Kept mouth element for positional tracking but removed all shape morph transitions
  - MorphSVGPlugin completely removed from project (no other code paths used it)
patterns_established:
  - Mouth element retained for positional face-tracking without shape morphing
observability_surfaces:
  - none
drill_down_paths:
  - .gsd/milestones/M002/slices/S01/tasks/T01-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T20:18:02.533Z
blocker_discovered: false
---

# S01: Fix Yeti Animation — Remove Mouth Morph & MorphSVGPlugin

**Removed mouth morph and MorphSVGPlugin — Yeti mouth permanently closed, 24KB frontend savings.**

## What Happened

Cleaned up animation.js by removing all mouth morph logic (morphSVG calls, mouthStatus tracking, tooth/tongue animations, eye-scale changes). The onEmailInput handler now simply calls getCoord() for eye tracking. Removed MorphSVGPlugin from index.html and deleted the 24KB plugin file. App launches cleanly with all remaining animations intact.

## Verification

App launched via npx tauri dev. No JS errors. Animation.js has no morph references. MorphSVGPlugin.min.js deleted.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

None.

## Known Limitations

None.

## Follow-ups

None.

## Files Created/Modified

- `src/animation.js` — Removed all mouth morph logic, morph variable declarations, and eye-scale transitions. Simplified onEmailInput to only call getCoord().
- `src/index.html` — Removed MorphSVGPlugin.min.js script tag and gsap.registerPlugin(MorphSVGPlugin) call.
- `src/MorphSVGPlugin.min.js` — Deleted — no longer needed.
