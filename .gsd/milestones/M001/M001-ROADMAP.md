# M001: 

## Vision
Rewrite the AutoIt character-by-character paste tool as a modern Tauri desktop app with an animated Yeti SVG character that watches text input and covers its eyes when masking is enabled. Compact UI with multi-slot clipboard and global hotkeys.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Tauri Scaffold + Yeti SVG Animation | medium | — | ✅ | App launches showing animated Yeti with text input. Eyes follow cursor as you type. |
| S02 | Core Paste Functionality (Character-by-Character) | high | S01 | ✅ | Enter text, click paste, text appears character-by-character in another window after 1.5s delay. |
| S03 | Mask Toggle + Yeti Eye Covering | low | S01 | ✅ | Click mask button → input becomes password field, Yeti covers eyes with arms. Click again → unmask, arms drop. |
| S04 | Multi-Slot Clipboard + Global Hotkeys | medium | S02 | ✅ | Store text in 3 slots, switch between them. Use global hotkey to trigger paste from any app. |
