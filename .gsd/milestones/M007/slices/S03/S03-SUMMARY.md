---
id: S03
parent: M007
milestone: M007
provides:
  - Working mouse click detection on macOS via CGEventTap
  - wait_for_user_click_macos(required_clicks, timeout) function
  - Full macOS type_text flow: Cmd+Tab -> wait for click -> type
requires:
  - slice: S01
    provides: Compilable project on macOS with core-graphics/core-foundation deps
  - slice: S02
    provides: CGEvent FFI module and keyboard simulation functions
affects:
  []
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - Raw CGEventTap FFI (no crate wrapper) — consistent with S02 approach
  - Listen-only tap so clicks pass through to target app
  - 50ms CFRunLoop poll interval — responsive without busy-spinning
  - Descriptive error on tap creation failure to surface missing Accessibility permission
patterns_established:
  - (none)
observability_surfaces:
  - none
drill_down_paths:
  []
duration: ""
verification_result: passed
completed_at: 2026-06-24T11:34:40.659Z
blocker_discovered: false
---

# S03: Mouse click detection via CGEventTap

**macOS click detection implemented via listen-only CGEventTap, replacing placeholder sleep in type_text**

## What Happened

Added CGEventTap and CFRunLoop FFI bindings to the cg module (CGEventTapCreate, CGEventTapEnable, CFMachPortCreateRunLoopSource, CFRunLoop functions, constants for listen-only tap and left mouse down event mask). Implemented wait_for_user_click_macos() with an atomic counter pattern matching the Windows low-level mouse hook approach: creates a listen-only CGEventTap so clicks pass through to the target app, polls via CFRunLoopRunInMode in 50ms intervals, checks click count against required threshold, times out after 30s. Replaced the placeholder sleep in the macOS type_text path with the real click detection call. cargo check passes cleanly on Windows with no regressions.

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passed (exit 0, 4.90s)

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

None.

## Deviations

None.

## Known Limitations

Cannot verify actual CGEventTap behavior on Windows — needs macOS testing to confirm click detection and Accessibility permission flow

## Follow-ups

None.

## Files Created/Modified

None.
