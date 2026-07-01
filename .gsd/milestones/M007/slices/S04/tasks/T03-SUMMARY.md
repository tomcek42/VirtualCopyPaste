---
id: T03
parent: S04
milestone: M007
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - (none)
duration: 
verification_result: passed
completed_at: 2026-06-24T11:38:44.892Z
blocker_discovered: false
---

# T03: Improved type_text error handling to emit accessibility-missing event when CGEventTap fails due to missing permission

**Improved type_text error handling to emit accessibility-missing event when CGEventTap fails due to missing permission**

## What Happened

In the macOS type_text path, replaced the bare ? operator on wait_for_user_click_macos with explicit error matching. If the error message contains "Accessibility", emits the accessibility-missing event to re-show the banner. The descriptive error from wait_for_user_click_macos still propagates to the frontend status area.

## Verification

cargo check passed

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo check --manifest-path src-tauri/Cargo.toml` | 0 | pass | 3470ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
