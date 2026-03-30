---
estimated_steps: 5
estimated_files: 1
skills_used: []
---

# T01: Add Cargo release profile and build

1. Add [profile.release] to Cargo.toml with: strip = true, lto = true, codegen-units = 1, opt-level = 's'
2. Run npx tauri build
3. Measure release .exe size
4. Measure NSIS installer size
5. If build fails with LTO, fall back to lto = 'thin' or lto = false

## Inputs

- `src-tauri/Cargo.toml`

## Expected Output

- `src-tauri/Cargo.toml (updated with release profile)`
- `src-tauri/target/release/bundle/nsis/*.exe (installer)`

## Verification

ls -la src-tauri/target/release/*.exe to verify size under 10MB. Launch installed app to verify it works.
