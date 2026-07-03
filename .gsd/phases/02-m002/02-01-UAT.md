# S01: Fix Yeti Animation — Remove Mouth Morph & MorphSVGPlugin — UAT

**Milestone:** M002
**Written:** 2026-03-29T20:18:02.534Z

## UAT: Yeti Animation Fix\n\n1. Start app with `npx tauri dev`\n2. Type text in the input field → eyes should follow the cursor position, mouth stays closed\n3. Click the mask toggle → arms should cover Yeti's eyes\n4. Click mask toggle again → arms uncover\n5. Click outside the input → face resets to center\n6. No JS console errors at any point
