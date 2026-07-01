---
id: T03
parent: S02
milestone: M007
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - (none)
duration: 
verification_result: passed
completed_at: 2026-06-24T11:29:36.427Z
blocker_discovered: false
---

# T03: Added enus_char_to_keycode_macos() mapping and send_vkey_char_enus_macos() for EN-US layout

**Added enus_char_to_keycode_macos() mapping and send_vkey_char_enus_macos() for EN-US layout**

## What Happened

Implemented enus_char_to_keycode_macos() with full EN-US keyboard mapping (macOS virtual keycodes, which differ from Windows scancodes — e.g. 'a' is 0x00 on macOS vs 0x1E on Windows). Covers number row, QWERTY row, home row, bottom row, special characters, space, and tab. Built send_vkey_char_enus_macos() that explicitly presses/releases Shift when needed with configurable intra-key delays, matching the Windows send_vkey_char_enus pattern. Falls back to send_unicode_char_macos() for unmapped characters.

## Verification

cargo check passes on Windows (exit 0)

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
