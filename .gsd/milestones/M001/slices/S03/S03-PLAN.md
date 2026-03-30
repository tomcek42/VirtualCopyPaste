# S03: Mask Toggle + Yeti Eye Covering

**Goal:** Add mask/unmask toggle that triggers Yeti arm animation and hides input text.
**Demo:** After this: Click mask button → input becomes password field, Yeti covers eyes with arms. Click again → unmask, arms drop.

## Tasks
- [x] **T01: Mask toggle button switches input type and triggers Yeti arm cover/uncover animation.** — Add a mask toggle button (eye icon) next to the text input. Wire it to: 1) toggle input type between 'text' and 'password', 2) call window.yetiAnimation.coverEyes() or uncoverEyes(), 3) update the button icon (👁️ / 🙈). Style the toggle button as a compact icon button overlaid on the right side of the input field.
  - Estimate: 20m
  - Files: src/index.html, src/styles.css, src/app.js
  - Verify: Click mask button — input becomes password field, Yeti covers eyes. Click again — unmask, eyes uncover.
