---
id: T01
parent: S04
milestone: M007
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - (none)
duration: 
verification_result: passed
completed_at: 2026-06-24T11:38:35.592Z
blocker_discovered: false
---

# T01: Added AXIsProcessTrusted FFI binding, check_accessibility_permission and open_accessibility_settings Tauri commands

**Added AXIsProcessTrusted FFI binding, check_accessibility_permission and open_accessibility_settings Tauri commands**

## What Happened

Added AXIsProcessTrusted FFI from ApplicationServices framework in the cg module. Created check_accessibility_permission command returning bool (true on non-macOS). Created open_accessibility_settings command that opens System Settings via x-apple.systempreferences URL scheme. Both registered in invoke_handler.

## Verification

cargo check passed (exit 0, 3.47s)

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
