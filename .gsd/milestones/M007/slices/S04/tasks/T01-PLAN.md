---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Added AXIsProcessTrusted FFI binding, check_accessibility_permission and open_accessibility_settings Tauri commands

Add AXIsProcessTrusted and AXIsProcessTrustedWithOptions FFI bindings behind cfg(target_os = macos). Create a check_accessibility_permission Tauri command that returns a boolean. Create an open_accessibility_settings Tauri command that opens System Settings > Privacy > Accessibility via the x-apple.systempreferences URL scheme.

## Inputs

- `src-tauri/src/main.rs (current cg module, invoke_handler registration)`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check passes, both commands registered in invoke_handler
