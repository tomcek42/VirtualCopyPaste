---
id: S01
parent: M003
milestone: M003
provides:
  - Clean emoji-free UI with SVG icons
requires:
  []
affects:
  - S02
key_files:
  - src/index.html
  - src/app.js
  - src/styles.css
  - src/eye.svg
  - src/eye-off.svg
  - src-tauri/icons/icon.ico
key_decisions:
  - SVG img element swap instead of inline SVG for simpler code
  - Used sharp for SVG-to-ICO since cairo unavailable on Windows
patterns_established:
  - SVG icons via img element swap for toggle states
observability_surfaces:
  - none
drill_down_paths:
  - .gsd/milestones/M003/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M003/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T04:53:53.246Z
blocker_discovered: false
---

# S01: Replace Emoji Icons with SVGs & App Icon

**All emoji replaced with SVG icons, app icon now uses textbox.svg.**

## What Happened

Replaced all emoji icons with SVG graphics. Mask toggle now swaps between eye.svg and eye-off.svg via img src. Paste button and status messages use plain text. Converted textbox.svg to multi-resolution icon.ico for the app icon. Zero emoji remaining in the frontend.

## Verification

App launched clean. No emoji in any frontend file. Icon.ico created with 4 sizes.

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

- `src/index.html` — Replaced emoji in mask toggle with SVG img element
- `src/app.js` — Toggle swaps eye.svg/eye-off.svg, removed emoji from status text
- `src/styles.css` — Added .mask-icon class, removed font-size from .mask-toggle
- `src/eye.svg` — Copied from project root for frontend serving
- `src/eye-off.svg` — Copied from project root for frontend serving
- `src-tauri/icons/icon.ico` — Replaced with textbox.svg conversion (16/32/48/256px)
