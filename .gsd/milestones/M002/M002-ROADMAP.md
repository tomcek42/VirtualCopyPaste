# M002: 

## Vision
Fix the Yeti animation to match the original (closed mouth, no morph), remove unnecessary MorphSVGPlugin dependency, and configure Cargo release profile for minimal binary size (~5-8MB instead of 216MB debug).

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | Fix Yeti Animation — Remove Mouth Morph & MorphSVGPlugin | low | — | ✅ | After this, the Yeti has a permanently closed mouth, eyes track text, arms cover on mask — MorphSVGPlugin is gone. |
| S02 | Release Build Profile & First Build | medium | S01 | ✅ | After this, npx tauri build produces a small installer (~3-5MB) and the app runs from the release binary. |
