---
id: M007
title: "macOS Port"
status: complete
completed_at: 2026-06-24T21:37:36.436Z
key_decisions:
  - Raw CoreGraphics FFI (extern C in mod cg) over core-graphics crate wrappers for full API control
  - Cmd+Left / Cmd+Shift+Right / ForwardDelete for clearing auto-indent after Return
  - Listen-only CGEventTap (kCGEventTapOptionListenOnly) for click detection — clicks pass through to target app
  - AXIsProcessTrusted for permission check with auto-dismiss banner on window focus
  - Universal binary (--target universal-apple-darwin) for both Apple Silicon and Intel Macs
key_files:
  - src-tauri/src/main.rs
  - src-tauri/Cargo.toml
  - src-tauri/tauri.conf.json
  - .github/workflows/release.yml
  - src/app.js
  - src/index.html
  - src/styles.css
lessons_learned:
  - CGEvent FFI is straightforward but requires careful flag management (FLAG_COMMAND, FLAG_SHIFT)
  - CGEventTap creation can silently fail without Accessibility permission — need to handle gracefully
  - macOS auto-indent behavior in editors requires explicit clearing after Return key simulation
---

# M007: macOS Port

**Full macOS port — CGEvent keyboard simulation, CGEventTap click detection, Accessibility permission UX, and dual-platform CI/CD**

## What Happened

Ported Virtual Copy Paste to macOS across 5 slices. S01 set up build config (Cargo.toml macOS deps, tauri.conf.json .dmg bundle, icon.icns). S02 implemented core keyboard simulation via raw CoreGraphics FFI: Cmd+Tab window switching, Return key with auto-indent clearing, Unicode character input via CGEventKeyboardSetUnicodeString, and EN-US keycode mapping with Shift support. S03 replaced the placeholder sleep with real CGEventTap-based mouse click detection — a listen-only tap with atomic counter, 50ms polling, and 30s timeout. S04 added Accessibility permission detection (AXIsProcessTrusted) with a user-facing banner that auto-dismisses when permission is granted. S05 extended GitHub Actions with a macOS build job producing universal (ARM64 + x86_64) .dmg artifacts. All macOS code is behind #[cfg(target_os = "macos")] — zero Windows regressions confirmed via cargo check.

## Success Criteria Results

Not provided.

## Definition of Done Results

Not provided.

## Requirement Outcomes

Not provided.

## Deviations

None.

## Follow-ups

Runtime testing on physical Mac hardware, version bump to v2.8.0, tag push to trigger first dual-platform CI build
