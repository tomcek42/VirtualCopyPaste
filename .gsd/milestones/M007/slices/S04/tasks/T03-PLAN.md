---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T03: Improved type_text error handling to emit accessibility-missing event when CGEventTap fails due to missing permission

In the macOS type_text path, catch the CGEventTap failure from wait_for_user_click_macos and return a user-friendly error mentioning Accessibility permission. Also emit the accessibility-missing event so the banner reappears if dismissed. Ensure the error propagates cleanly to the frontend status area.

## Inputs

- `src-tauri/src/main.rs (type_text macOS block, wait_for_user_click_macos error)`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check passes, error path returns descriptive message
