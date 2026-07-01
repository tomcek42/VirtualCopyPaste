---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T03: Added enus_char_to_keycode_macos() mapping and send_vkey_char_enus_macos() for EN-US layout

Add enus_char_to_keycode_macos() mapping EN-US characters to macOS virtual keycodes, and send_vkey_char_enus_macos() that sends key events with shift modifier via CGEvent. Falls back to send_unicode_char_macos for unmapped chars.

## Inputs

- `Windows enus_char_to_scancode and send_vkey_char_enus as reference`
- `macOS virtual keycode table`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passes (exit 0)
