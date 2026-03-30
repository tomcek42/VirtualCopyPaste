# S01: Tauri Scaffold + Yeti SVG Animation

**Goal:** Get Tauri project running with the Yeti SVG animation integrated and responding to text input.
**Demo:** After this: App launches showing animated Yeti with text input. Eyes follow cursor as you type.

## Tasks
- [x] **T01: Tauri v2 project scaffolded with compact window, dark theme, and working cargo build.** — Create the Tauri v2 project structure using cargo create-tauri-app or manual init. Set up package.json with dev dependencies (GSAP via CDN or npm). Configure Tauri window to be compact (~420x550px), always-on-top capable, with a clean title. Create the basic HTML entry point with a text input field styled for the app.
  - Estimate: 20m
  - Files: package.json, src-tauri/Cargo.toml, src-tauri/tauri.conf.json, src-tauri/src/main.rs, src/index.html, src/styles.css
  - Verify: cd src-tauri && cargo build 2>&1 | tail -5; npm run tauri dev launches the window
- [x] **T02: Yeti SVG with all animated elements ported into the Tauri frontend.** — Extract the Yeti SVG markup from the vYetti project's unpentry.jsp. Clean it up — remove vSphere-specific elements, keep the animated SVG with all named classes (eyeL, eyeR, armL, armR, mouth, face, etc.). Port the app.component.css styles, adapting colors/sizing for our compact dark-themed UI. Include GSAP via CDN (TweenMax). Ensure the Yeti renders centered above the text input.
  - Estimate: 30m
  - Files: src/index.html, src/styles.css
  - Verify: npm run tauri dev — Yeti SVG renders visually in the window above the input field
- [x] **T03: Yeti animation JS wired up with eye tracking, mouth reactions, and arm cover/uncover API.** — Adapt login-animation.js from the vYetti project. Wire the eye-tracking logic to follow the caret position in our text input. Wire mouth size changes based on text length (small/medium/large). Handle focus/blur events. Replace morphSVG calls with CSS transitions or simple TweenMax transforms since morphSVG requires a paid GSAP plugin. Test that eyes follow text input and mouth reacts.
  - Estimate: 30m
  - Files: src/animation.js, src/index.html
  - Verify: npm run tauri dev — type text in input, Yeti eyes follow cursor position, mouth changes size
