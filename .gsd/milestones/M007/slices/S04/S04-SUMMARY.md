---
id: S04
parent: M007
milestone: M007
provides:
  - (none)
requires:
  []
affects:
  []
key_files:
  - src-tauri/src/main.rs
  - src/app.js
  - src/index.html
  - src/styles.css
key_decisions:
  - Used AXIsProcessTrusted from ApplicationServices framework for permission check
  - Banner design: fixed-bottom, red-tinted, with one-click Grant Access button
  - Auto-dismiss on window focus when permission is granted — no app restart needed
patterns_established:
  - (none)
observability_surfaces:
  - none
drill_down_paths:
  []
duration: ""
verification_result: passed
completed_at: 2026-06-24T21:36:09.896Z
blocker_discovered: false
---

# S04: Accessibility permission check and user guidance

**macOS Accessibility permission detection with user-facing banner and one-click grant flow**

## What Happened

Added `AXIsProcessTrusted()` FFI binding to the `cg` module. Two new Tauri commands: `check_accessibility_permission` (returns bool, always true on non-macOS) and `open_accessibility_settings` (opens System Settings at the Accessibility pane via `x-apple.systempreferences:` URL scheme). On startup, if not trusted, emits `accessibility-missing` event. The frontend shows a fixed-bottom red-tinted banner with "Grant Access" button. On window focus, re-checks permission and auto-hides the banner when granted. The `type_text` macOS path also emits `accessibility-missing` if `wait_for_user_click_macos` fails with an Accessibility error, so the user gets guidance even if they dismiss the banner. Window height calculation accounts for the banner.

## Verification

cargo check passed (exit 0, 3.35s) — no compile errors, no Windows regressions

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

None.

## Follow-ups

None.

## Files Created/Modified

None.
