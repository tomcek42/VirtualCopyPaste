---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T03: Replaced placeholder sleep in macOS type_text with real wait_for_user_click_macos() call

In the macOS cfg block of type_text, replace the placeholder sleep (lines 984-986) with a call to wait_for_user_click_macos(required_clicks, Duration::from_secs(30)), matching the Windows pattern. Use required_clicks=2 for double-click mode, 1 for single-click. Propagate errors with ?.

## Inputs

- `src-tauri/src/main.rs (type_text macOS block, wait_for_user_click_macos from T02)`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check --manifest-path src-tauri/Cargo.toml passes with exit 0, no warnings in macOS-related code
