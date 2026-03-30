# Decisions Register

<!-- Append-only. Never edit or remove existing rows.
     To reverse a decision, add a new row that supersedes it.
     Read this file at the start of any planning or research phase. -->

| # | When | Scope | Decision | Choice | Rationale | Revisable? | Made By |
|---|------|-------|----------|--------|-----------|------------|---------|
| D001 |  | architecture | Technology stack for Virtual Copy Paste v2 | Tauri v2 (Rust backend + HTML/CSS/JS frontend) | The Yeti SVG animation requires a web-capable UI. Tauri produces small binaries (~5MB vs 150MB+ Electron), is fast, and the Rust backend can handle system-level keyboard simulation. User selected from 3 options. | Yes | collaborative |
| D002 |  | ui | UI animation approach for Yeti character | Adapt the vYetti SVG animation (GSAP/TweenMax) from William Lam's vyetti-vsphere-client-customization project | The original Yeti animation by Darin S uses SVG + GSAP for eye-tracking on text input and arm-covering on password focus. This maps perfectly to our use case: eyes follow text input, arms cover eyes when masking is toggled. | Yes | human |
| D003 | M003 | architecture | How to ensure focus in target application after Alt+Tab window switch | Save cursor position before Alt+Tab, click at that position after window switch to ensure input focus in target app | Alt+Tab alone activates the window but doesn't guarantee cursor focus in the input area (e.g. console windows require a click). User positions cursor in target first, then switches to VCP — clicking back at the saved position restores focus exactly where intended. | Yes | collaborative |
