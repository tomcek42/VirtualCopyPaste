# S01: Replace Emoji Icons with SVGs & App Icon

**Goal:** Replace emoji-based UI elements with SVG graphics for a cleaner, more professional look.
**Demo:** After this: After this, the mask toggle shows clean SVG eye icons and the app has a proper textbox icon in the taskbar.

## Tasks
- [x] **T01: Replaced all emoji icons with SVG graphics — mask toggle uses eye.svg/eye-off.svg, no emoji remain.** — 1. Replace emoji text in maskToggle button with inline SVG img elements
2. In app.js handleMaskToggle(): swap between eye.svg (unmask state) and eye-off.svg (masked state)
3. Update initial button content to show eye.svg
4. Style the SVG img to fit the button (size, color, alignment)
5. Replace emoji in pasteBtn with clean text or SVG icon
6. Update styles.css for the new icon sizing
  - Estimate: 10min
  - Files: src/index.html, src/app.js, src/styles.css
  - Verify: npx tauri dev — verify mask toggle shows SVG eye icons, paste button has no emoji
- [x] **T02: Converted textbox.svg to icon.ico with 4 resolution sizes (16-256px).** — 1. Convert textbox.svg to multi-resolution icon.ico (16, 32, 48, 256px) using a tool or manual process
2. Replace src-tauri/icons/icon.ico with the new icon
3. Verify Tauri picks up the new icon in dev mode
  - Estimate: 10min
  - Files: src-tauri/icons/icon.ico
  - Verify: npx tauri dev — verify app icon in taskbar and title bar shows textbox icon
