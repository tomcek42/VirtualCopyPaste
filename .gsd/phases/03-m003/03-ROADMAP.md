# M003: M003

**Vision:** Refine the UI for a subtler, more elegant look — replace emoji icons with SVG graphics, set a proper app icon, and implement smart window-switch with click-to-focus so text reliably arrives in the target application.

## Success Criteria

- Mask toggle uses eye.svg / eye-off.svg instead of emoji
- App icon is based on textbox.svg
- Paste action: Alt+Tab to previous window, click at remembered cursor position, then type
- Text arrives correctly in target applications including console windows
- No regressions in existing functionality (Yeti animation, hotkeys)

## Slices

- [x] **S01: Replace Emoji Icons with SVGs & App Icon** `risk:low` `depends:[]`
  > After this: 

- [x] **S02: Smart Focus Paste — Click at Saved Cursor Position** `risk:medium` `depends:[S01]`
  > After this: 

## Boundary Map

```\nIN: index.html, styles.css, app.js, main.rs, eye.svg, eye-off.svg, textbox.svg\nOUT: animation.js (no changes), Cargo.toml (may need mouse API features)\n```
