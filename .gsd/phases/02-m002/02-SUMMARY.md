---
id: S02
parent: M002
milestone: M002
provides:
  - Working release build pipeline
  - NSIS installer at src-tauri/target/release/bundle/nsis/
requires:
  - slice: S01
    provides: Clean frontend without MorphSVGPlugin
affects:
  []
key_files:
  - src-tauri/Cargo.toml
key_decisions:
  - Aggressive release profile: strip + full LTO + single codegen unit + size optimization
patterns_established:
  - Cargo release profile with strip/lto/codegen-units=1/opt-level=s for minimal Tauri binary size
observability_surfaces:
  - none
drill_down_paths:
  - .gsd/milestones/M002/slices/S02/tasks/T01-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T20:36:25.885Z
blocker_discovered: false
---

# S02: Release Build Profile & First Build

**Release build produces 4.3MB exe and 1.4MB installer — 98% reduction from debug build.**

## What Happened

Added Cargo release profile with maximum size optimization. First build attempt timed out at 10 min due to LTO link phase. Second attempt completed in ~5 min. Final sizes: 4.3MB .exe (98% reduction from 206MB debug), 1.4MB NSIS installer. Both well under the 10MB target.

## Verification

Release build completed. Exe is 4.3MB, installer is 1.4MB. Both under 10MB target.

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

LTO build takes ~5 minutes on this machine. Acceptable for release builds.

## Follow-ups

Fix bundle identifier warning (com.virtualcopypaste.app → com.virtualcopypaste.vcp or similar).

## Files Created/Modified

- `src-tauri/Cargo.toml` — Added [profile.release] with strip=true, lto=true, codegen-units=1, opt-level='s'
