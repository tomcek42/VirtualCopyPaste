# S01: Tauri Scaffold + Yeti SVG Animation — UAT

**Milestone:** M001
**Written:** 2026-03-28T23:46:49.034Z

## S01 UAT: Tauri Scaffold + Yeti SVG Animation\n\n### Test 1: App Launch\n- [ ] Run `npx tauri dev`\n- [ ] App window appears (~420x550px)\n- [ ] Yeti SVG character is visible in a circular frame\n\n### Test 2: Eye Tracking\n- [ ] Click into the text input field\n- [ ] Type text slowly\n- [ ] Yeti eyes follow the cursor position left-to-right\n\n### Test 3: Mouth Reactions\n- [ ] With empty input, mouth is small\n- [ ] Type 1-8 characters, mouth grows to medium\n- [ ] Type 9+ characters, mouth grows to large\n- [ ] Delete all text, mouth returns to small\n\n### Test 4: Focus/Blur\n- [ ] Click away from the input field\n- [ ] Yeti face resets to center position
