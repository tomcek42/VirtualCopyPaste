---
id: M001
title: "Virtual Copy Paste v2 — Tauri Rewrite with Yeti Animation"
status: complete
completed_at: 2026-03-28T23:58:59.022Z
key_decisions:
  - Tauri v2 as the application framework (D001)
  - vYetti SVG animation adapted for GSAP v3 free tier (D002)
  - Replaced paid morphSVG with SVG path attribute animation
  - Windows SendInput with KEYEVENTF_UNICODE for full Unicode support
  - Rust-side global shortcut registration with event emission to frontend
  - Static HTML frontend (no build step/bundler)
key_files:
  - src/index.html
  - src/styles.css
  - src/animation.js
  - src/app.js
  - src-tauri/src/main.rs
  - src-tauri/tauri.conf.json
  - src-tauri/capabilities/default.json
  - README.md
lessons_learned:
  - GSAP v3 can animate SVG path 'd' attributes directly via attr:{d} — no need for the paid morphSVG plugin
  - Tauri v2 static HTML apps need withGlobalTauri:true for JS API access
  - Global shortcut plugin works best when registered from Rust setup() for static HTML apps
  - The windows crate v0.61 is already a Tauri dependency — no version conflicts for SendInput
---

# M001: Virtual Copy Paste v2 — Tauri Rewrite with Yeti Animation

**Virtual Copy Paste v2 — Tauri app with animated Yeti, character-by-character paste, mask toggle, 3 clipboard slots, and global hotkeys.**

## What Happened

Rewrote the original AutoIt Virtual Copy Paste tool as a modern Tauri v2 desktop application. The app features an animated Yeti SVG character (originally by Darin Senneff, adapted from William Lam's vYetti project) that watches the user's text input — eyes follow the caret position using trigonometry, mouth changes size with text length, and arms cover the eyes when the mask toggle is activated. The core paste functionality uses Windows SendInput API with KEYEVENTF_UNICODE to type text character-by-character into any target window, with a 2-second countdown for window switching. Three clipboard slots allow storing different text snippets, and global hotkeys (Ctrl+Shift+1/2/3) enable pasting from any application without switching back to the tool. The entire app is 926 lines of code across 5 source files.

## Success Criteria Results

All 7 success criteria passed — see M001-VALIDATION.md for details.

## Definition of Done Results

- ✅ Tauri app builds successfully on Windows — cargo build, 0 warnings\n- ✅ Yeti SVG animation renders and responds to input — eyes track, mouth changes\n- ✅ Mask toggle makes Yeti cover eyes and hides text — coverEyes/uncoverEyes GSAP animation\n- ✅ Character-by-character paste works into target windows — SendInput with KEYEVENTF_UNICODE\n- ✅ Multi-slot clipboard stores and recalls text — 3 slots with auto-save and content indicators\n- ✅ Global hotkeys trigger paste — Ctrl+Shift+1/2/3 registered via tauri-plugin-global-shortcut\n- ✅ Window is compact (≤ 450px wide) — 420x550px

## Requirement Outcomes

All user requirements delivered:\n- Yeti animation from vYetti project: ✅ Implemented\n- Mask toggle with eye-covering: ✅ Implemented\n- Multi-slot clipboard: ✅ 3 slots with auto-save\n- Global hotkeys: ✅ Ctrl+Shift+1/2/3\n- Compact UI: ✅ 420x550px

## Deviations

None.

## Follow-ups

None.
