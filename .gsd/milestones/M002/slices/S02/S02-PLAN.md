# S02: Release Build Profile & First Build

**Goal:** Add Cargo release profile optimizations and produce first working release build.
**Demo:** After this: After this, npx tauri build produces a small installer (~3-5MB) and the app runs from the release binary.

## Tasks
- [x] **T01: Release build configured — 4.3MB .exe and 1.4MB installer (98% reduction from 206MB debug).** — 1. Add [profile.release] to Cargo.toml with: strip = true, lto = true, codegen-units = 1, opt-level = 's'
2. Run npx tauri build
3. Measure release .exe size
4. Measure NSIS installer size
5. If build fails with LTO, fall back to lto = 'thin' or lto = false
  - Estimate: 20min (mostly build time)
  - Files: src-tauri/Cargo.toml
  - Verify: ls -la src-tauri/target/release/*.exe to verify size under 10MB. Launch installed app to verify it works.
