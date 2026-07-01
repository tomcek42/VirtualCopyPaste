---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T04: Wired macOS keyboard functions into type_text command, replacing the error stub

Replace the macOS error stub in type_text with the real implementation: call cmd_tab, sleep placeholder for click wait (S03 will add CGEventTap detection), emit paste-status events, then type characters using the selected mode. Handle \r\n normalization same as Windows path.

## Inputs

- `All T01-T03 macOS functions`
- `Windows type_text path as reference`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passes (exit 0)
