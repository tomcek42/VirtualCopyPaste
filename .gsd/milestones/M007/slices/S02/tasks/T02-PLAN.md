---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T02: Added send_unicode_char_macos() using CGEventKeyboardSetUnicodeString FFI

Add send_unicode_char_macos() using CGEvent with CGEventKeyboardSetUnicodeString. Handles newlines by delegating to send_enter_macos. Handles UTF-16 surrogate pairs for supplementary characters.

## Inputs

- `Windows send_unicode_char as reference`
- `core-graphics CGEvent API`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passes (exit 0)
