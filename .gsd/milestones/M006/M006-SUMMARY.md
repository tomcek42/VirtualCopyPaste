---
id: M006
title: "Clipboard Auto-Paste on Window Focus"
status: complete
completed_at: 2026-07-01T09:50:28.258Z
key_decisions:
  - Use window focus event as trigger (works with all activation methods)
  - Use navigator.clipboard.readText() JS API instead of Rust-side clipboard crate
  - Two replace modes: always (default) and only-when-empty
key_files:
  - src/app.js
  - src/settings.js
  - src/settings.html
lessons_learned:
  - Clipboard API works reliably in Tauri v2 webviews on Windows — no Rust fallback needed
---

# M006: Clipboard Auto-Paste on Window Focus

**Automatic clipboard paste into single-line input on window focus, released in v2.7.0.**

## What Happened

This milestone added automatic clipboard pasting when the VCP window gains focus in single-line mode. Configurable via Settings with two modes: always replace or only when empty. Uses the navigator.clipboard.readText() API. The feature was fully implemented, tested, committed (6af7344), and released as part of v2.7.0. GSD tracked the discuss phase (CONTEXT + ROADMAP) but the plan/execute phases were done inline without generating task-level artifacts.

## Success Criteria Results

Not provided.

## Definition of Done Results

Not provided.

## Requirement Outcomes

Not provided.

## Deviations

None.

## Follow-ups

None.
