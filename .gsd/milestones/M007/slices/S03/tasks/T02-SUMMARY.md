---
id: T02
parent: S03
milestone: M007
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - Listen-only tap (kCGEventTapOptionListenOnly) so clicks pass through to target app — matches Windows hook behavior
  - 50ms CFRunLoop poll interval — responsive without busy-spinning
  - Same pattern as Windows: atomic counter + required_clicks + timeout
duration: 
verification_result: passed
completed_at: 2026-06-24T11:34:15.179Z
blocker_discovered: false
---

# T02: Implemented wait_for_user_click_macos() with atomic counter and listen-only CGEventTap

**Implemented wait_for_user_click_macos() with atomic counter and listen-only CGEventTap**

## What Happened

Added CLICK_COUNT_MACOS AtomicU32 static (paralleling Windows CLICK_COUNT). Implemented mouse_tap_callback extern C function that increments the counter on kCGEventLeftMouseDown. Implemented wait_for_user_click_macos(required_clicks, timeout) that: resets counter to 0, creates a listen-only CGEventTap (clicks pass through to target app), creates a CFRunLoopSource, adds it to the current thread's run loop, polls in 50ms intervals via CFRunLoopRunInMode checking counter and timeout, then cleans up (remove source, invalidate port, release both). Returns descriptive errors for tap creation failure (missing Accessibility permission) and timeout. Changed the AtomicU32/Ordering import to use cfg(any(windows, target_os = "macos")) so both platforms share the import.

## Verification

cargo check passed (exit 0, 4.90s)

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo check --manifest-path src-tauri/Cargo.toml` | 0 | pass | 4900ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
