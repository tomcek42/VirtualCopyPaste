# M001: M001

**Vision:** Rewrite the AutoIt character-by-character paste tool as a modern Tauri desktop app with an animated Yeti SVG character that watches text input and covers its eyes when masking is enabled. Compact UI with multi-slot clipboard and global hotkeys.

## Success Criteria

- App launches as a compact Tauri window with Yeti SVG animation
- Yeti eyes follow text input cursor position
- Yeti covers eyes when mask toggle is activated
- Text is typed character-by-character into the active window after a delay
- Multi-slot clipboard allows storing and recalling multiple text snippets
- Global hotkeys trigger paste without switching windows
- Window stays small and unobtrusive

## Slices

- [x] **S01: Tauri Scaffold + Yeti SVG Animation** `risk:medium` `depends:[]`
  > After this: 

- [x] **S02: Core Paste Functionality (Character-by-Character)** `risk:high` `depends:[S01]`
  > After this: 

- [x] **S03: Mask Toggle + Yeti Eye Covering** `risk:low` `depends:[S01]`
  > After this: 

- [x] **S04: Multi-Slot Clipboard + Global Hotkeys** `risk:medium` `depends:[S02]`
  > After this: 

## Boundary Map

```\nTauri App\n├── Frontend (HTML/CSS/JS)\n│   ├── Yeti SVG Animation (GSAP)\n│   ├── Text Input Field\n│   ├── Mask Toggle Button\n│   └── Clipboard Slot Selector\n└── Backend (Rust)\n    ├── Keyboard Simulation (SendInput)\n    ├── Global Hotkey Registration\n    └── Clipboard Slot Storage\n```
