---
id: M002
title: "Yeti Animation Fix & Release Build Optimization"
status: complete
completed_at: 2026-03-30T04:49:09.893Z
key_decisions:
  - Kept mouth element for positional face-tracking but removed all shape morph transitions
  - MorphSVGPlugin completely removed — no other code paths used it
  - Aggressive release profile: strip=true, lto=true, codegen-units=1, opt-level='s'
key_files:
  - src/animation.js
  - src/index.html
  - src-tauri/Cargo.toml
lessons_learned:
  - Tauri debug builds are ~200MB — always use release builds for size evaluation
  - Full LTO with codegen-units=1 takes ~5 min but produces 4.3MB exe vs 206MB debug
  - MorphSVGPlugin is now 100% free (Webflow acquired GSAP April 2025) — no license concerns
---

# M002: Yeti Animation Fix & Release Build Optimization

**Fixed Yeti animation (permanently closed mouth) and configured release build (4.3MB exe, 1.4MB installer).**

## What Happened

Two slices completed. S01 cleaned up the Yeti animation by removing all mouth morph logic (morphSVG calls, mouthStatus tracking, tooth/tongue/eye-scale transitions) and deleting MorphSVGPlugin.min.js (24KB). The onEmailInput handler was simplified to just call getCoord() for eye tracking. S02 added a Cargo release profile with aggressive size optimization (strip, LTO, single codegen unit, size-optimized). The release build produced a 4.3MB .exe and 1.4MB NSIS installer — a 98% reduction from the 206MB debug build.

## Success Criteria Results

- ✅ Yeti mouth stays closed at all times — morph code fully removed\n- ✅ Eye-tracking follows text cursor — preserved in getCoord()\n- ✅ Arms cover eyes on mask toggle — coverEyes/uncoverEyes intact\n- ✅ MorphSVGPlugin.min.js removed from project — deleted\n- ✅ Release .exe under 10MB — 4.3MB\n- ✅ NSIS installer builds — 1.4MB\n- ✅ App runs from release build — confirmed

## Definition of Done Results

- ✅ Mouth morph code removed from animation.js — all morphSVG calls, mouthStatus tracking, tooth/tongue animations deleted\n- ✅ MorphSVGPlugin.min.js deleted and script tag removed from index.html\n- ✅ Eye-tracking and arm-cover animations still work — getCoord(), coverEyes(), uncoverEyes() preserved\n- ✅ Cargo.toml has release profile with strip/lto/opt-level — [profile.release] added\n- ✅ npx tauri build produces working installer — 1.4MB NSIS installer created\n- ✅ Release .exe under 10MB — 4.3MB actual (98% reduction from 206MB debug)

## Requirement Outcomes

No formal requirements tracked for this milestone. All changes driven by user feedback: Yeti mouth now matches the original (closed), binary size is production-ready.

## Deviations

None.

## Follow-ups

Fix bundle identifier warning (com.virtualcopypaste.app ends with .app, conflicts with macOS convention). Not relevant for Windows-only tool but worth cleaning up.
