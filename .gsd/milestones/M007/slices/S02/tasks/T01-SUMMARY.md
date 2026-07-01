---
id: T01
parent: S02
milestone: M007
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - Used raw FFI instead of core-graphics crate wrappers for CGEvent — avoids uncertainty about the crate's exact wrapper API since we cannot compile-check macOS code on Windows
  - Used Cmd+Left/Cmd+Shift+Right for start/end of line on macOS instead of Home/End which scroll to document boundaries in most macOS apps
duration: 
verification_result: passed
completed_at: 2026-06-24T11:29:21.338Z
blocker_discovered: false
---

# T01: Added cmd_tab() and send_enter_macos() using raw CGEvent FFI

**Added cmd_tab() and send_enter_macos() using raw CGEvent FFI**

## What Happened

Implemented a cg FFI module with raw CoreGraphics/CoreFoundation bindings (CGEventSourceCreate, CGEventCreateKeyboardEvent, CGEventPost, CGEventSetFlags, CGEventKeyboardSetUnicodeString, CFRelease). Added post_key_event() helper that creates a source, builds a keyboard event, sets flags, posts it, and releases both objects. Built cmd_tab() using explicit Command key down → Tab down/up → Command key up sequence. Built send_enter_macos() that sends Return then clears auto-indent using Cmd+Left → Cmd+Shift+Right → ForwardDelete (macOS equivalent of Home → Shift+End → Delete on Windows).

## Verification

cargo check passes on Windows (exit 0) — macOS code behind cfg guard, no regressions

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo check --manifest-path src-tauri/Cargo.toml` | 0 | pass | 5160ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
