# S02: Release Build Profile & First Build — UAT

**Milestone:** M002
**Written:** 2026-03-29T20:36:25.886Z

## UAT: Release Build\n\n1. Run `npx tauri build`\n2. Verify release .exe exists at `src-tauri/target/release/virtual-copy-paste.exe`\n3. Verify .exe is under 10MB (actual: 4.3MB)\n4. Verify NSIS installer exists at `src-tauri/target/release/bundle/nsis/`\n5. Install the app via the NSIS installer\n6. Launch the installed app — verify Yeti animation, text input, mask toggle all work
