---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Added core-graphics and core-foundation as macOS-conditional dependencies

Add core-graphics and core-foundation crates under [target.'cfg(target_os = "macos")'.dependencies]. These are needed for CGEvent keyboard simulation and CGEventTap mouse hooks in later slices.

## Inputs

- `src-tauri/Cargo.toml`

## Expected Output

- `src-tauri/Cargo.toml`

## Verification

cargo check (or manual review that the dependency block is syntactically correct)
