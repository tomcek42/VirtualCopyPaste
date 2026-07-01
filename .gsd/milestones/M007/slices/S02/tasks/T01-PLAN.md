---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Added cmd_tab() and send_enter_macos() using raw CGEvent FFI

Add cmd_tab() using CGEvent to simulate Cmd+Tab window switching. Add send_enter_macos() to simulate Return key via CGEvent. Both behind #[cfg(target_os = macos)].

## Inputs

- `S01 macOS dependencies (core-graphics, core-foundation)`
- `Windows alt_tab and send_enter as reference`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passes (exit 0)
