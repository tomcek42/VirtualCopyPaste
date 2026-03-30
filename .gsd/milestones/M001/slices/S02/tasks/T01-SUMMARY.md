---
id: T01
parent: S02
milestone: M001
provides: []
requires: []
affects: []
key_files: ["src-tauri/src/main.rs", "src-tauri/Cargo.toml"]
key_decisions: ["Used windows crate v0.61 (already a Tauri dependency) for SendInput FFI", "KEYBDINPUT with KEYEVENTF_UNICODE for full Unicode character support including emoji", "UTF-16 encoding handles surrogate pairs for characters outside BMP"]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "cargo build succeeds with 0 warnings."
completed_at: 2026-03-28T23:49:02.596Z
blocker_discovered: false
---

# T01: Rust type_text command sends characters via Windows SendInput with Unicode support.

> Rust type_text command sends characters via Windows SendInput with Unicode support.

## What Happened
---
id: T01
parent: S02
milestone: M001
key_files:
  - src-tauri/src/main.rs
  - src-tauri/Cargo.toml
key_decisions:
  - Used windows crate v0.61 (already a Tauri dependency) for SendInput FFI
  - KEYBDINPUT with KEYEVENTF_UNICODE for full Unicode character support including emoji
  - UTF-16 encoding handles surrogate pairs for characters outside BMP
duration: ""
verification_result: passed
completed_at: 2026-03-28T23:49:02.597Z
blocker_discovered: false
---

# T01: Rust type_text command sends characters via Windows SendInput with Unicode support.

**Rust type_text command sends characters via Windows SendInput with Unicode support.**

## What Happened

Implemented the type_text Tauri command using Windows SendInput API. Each character is sent as a Unicode keypress (KEYEVENTF_UNICODE) with key-down and key-up events. Supports full Unicode including emoji via UTF-16 surrogate pair encoding. Configurable delay between characters (default 20ms) and initial delay (default 1500ms).

## Verification

cargo build succeeds with 0 warnings.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cd src-tauri && cargo build` | 0 | ✅ pass — 0 warnings | 15650ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
- `src-tauri/Cargo.toml`


## Deviations
None.

## Known Issues
None.
