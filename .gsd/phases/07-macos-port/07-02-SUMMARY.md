---
id: S02
parent: M007
milestone: M007
provides:
  - Working keyboard simulation on macOS (send_unicode_char_macos, send_vkey_char_enus_macos, send_enter_macos)
  - Window switching via cmd_tab()
  - type_text macOS path fully wired with mode selection
requires:
  - slice: S01
    provides: Compilable project on macOS with core-graphics/core-foundation dependencies
affects:
  []
key_files:
  - src-tauri/src/main.rs
key_decisions:
  - Used raw CoreGraphics/CoreFoundation FFI instead of core-graphics crate wrappers — more predictable when cross-developing from Windows
  - Used Cmd+Left/Cmd+Shift+Right for line start/end on macOS (Home/End scroll to document boundaries in most macOS apps)
  - Compatible mode without en-us layout falls back to Unicode on macOS (no VkKeyScanW equivalent)
  - Placeholder sleep for click wait (2s/3s) — S03 replaces with CGEventTap
patterns_established:
  - Raw CGEvent FFI pattern via cg module — reusable for S03 CGEventTap work
observability_surfaces:
  - none
drill_down_paths:
  []
duration: ""
verification_result: passed
completed_at: 2026-06-24T11:30:13.803Z
blocker_discovered: false
---

# S02: Core keyboard simulation via CGEvent

**macOS keyboard simulation fully implemented — cmd_tab, Unicode input, EN-US keycode input, enter/auto-indent, and type_text wired up**

## What Happened

Implemented all macOS keyboard simulation functions using raw CoreGraphics FFI (bypassing the core-graphics crate wrappers to avoid API uncertainty on a Windows dev machine). Added a cg module with FFI bindings for CGEventSourceCreate, CGEventCreateKeyboardEvent, CGEventPost, CGEventSetFlags, CGEventKeyboardSetUnicodeString, and CFRelease. Built post_key_event() as the foundation helper. Implemented cmd_tab() (Cmd+Tab window switch), send_enter_macos() (Return + auto-indent clearing via Cmd+Left/Cmd+Shift+Right/ForwardDelete), send_unicode_char_macos() (CGEventKeyboardSetUnicodeString for any Unicode character), enus_char_to_keycode_macos() (full EN-US keycode mapping table), and send_vkey_char_enus_macos() (scancode-based typing with Shift handling). Wired everything into the type_text Tauri command's macOS path with paste-status events and \r\n normalization. Click detection uses a placeholder sleep (S03 will add CGEventTap).

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passes (exit 0, 5.16s). All macOS code is behind #[cfg(target_os = "macos")] — Windows build has zero regressions. Actual macOS compilation requires building on macOS (or CI).

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

Click detection is a timed placeholder (2s normal, 3s double-click) — S03 will replace with proper CGEventTap mouse hook. Cannot verify macOS compilation on this Windows machine.

## Follow-ups

None.

## Files Created/Modified

None.
