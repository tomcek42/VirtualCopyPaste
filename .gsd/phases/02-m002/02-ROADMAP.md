# M002: M002

**Vision:** Fix the Yeti animation to match the original (closed mouth, no morph), remove unnecessary MorphSVGPlugin dependency, and configure Cargo release profile for minimal binary size (~5-8MB instead of 216MB debug).

## Success Criteria

- Yeti mouth stays closed at all times — no morph transitions on text input
- Eye-tracking still follows text cursor position
- Arms still cover eyes on mask toggle
- MorphSVGPlugin.min.js removed from project
- Release build produces .exe under 10MB
- NSIS installer builds successfully
- App runs correctly from release build

## Slices

- [x] **S01: Fix Yeti Animation — Remove Mouth Morph & MorphSVGPlugin** `risk:low` `depends:[]`
  > After this: 

- [x] **S02: Release Build Profile & First Build** `risk:medium` `depends:[S01]`
  > After this: 

## Boundary Map

```\nIN: animation.js, index.html, Cargo.toml, tauri.conf.json\nOUT: app.js (no changes needed), main.rs (no changes needed)\n```
