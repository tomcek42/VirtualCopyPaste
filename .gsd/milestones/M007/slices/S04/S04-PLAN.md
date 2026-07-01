# S04: Accessibility permission check and user guidance

**Goal:** Detect missing macOS Accessibility permission on startup and during paste, show a non-blocking banner guiding the user to grant it, and gracefully handle the permission-denied case without crashing.
**Demo:** Fresh install on macOS: app launches, detects missing Accessibility permission, shows a non-blocking banner or dialog explaining how to grant it. After user grants permission in System Settings, the next paste attempt works without app restart.

## Must-Haves

- 1. On macOS, app checks Accessibility permission on startup via AXIsProcessTrusted. 2. If missing, a non-blocking banner appears explaining how to grant it. 3. Banner has a button that opens System Settings > Privacy > Accessibility. 4. type_text returns a clear error when CGEventTap fails due to missing permission. 5. After granting permission, next paste attempt works without app restart. 6. cargo check passes on Windows with no regressions.

## Verification

- Run the task and slice verification checks for this slice.

## Tasks

- [x] **T01: Added AXIsProcessTrusted FFI binding, check_accessibility_permission and open_accessibility_settings Tauri commands** `est:20min`
  Add AXIsProcessTrusted and AXIsProcessTrustedWithOptions FFI bindings behind cfg(target_os = macos). Create a check_accessibility_permission Tauri command that returns a boolean. Create an open_accessibility_settings Tauri command that opens System Settings > Privacy > Accessibility via the x-apple.systempreferences URL scheme.
  - Files: `src-tauri/src/main.rs`
  - Verify: cargo check passes, both commands registered in invoke_handler

- [x] **T02: Added startup accessibility check, frontend banner with Grant Access button, and auto-dismiss on focus when permission granted** `est:25min`
  In the Tauri setup hook (macOS only), check AXIsProcessTrusted and emit an accessibility-missing event if false. In the frontend, listen for this event and show a non-blocking banner (similar to the update notice) with text explaining the permission and a button to open System Settings. Add corresponding HTML element and CSS styling.
  - Files: `src-tauri/src/main.rs`, `src/app.js`, `src/index.html`, `src/styles.css`
  - Verify: cargo check passes, HTML/CSS/JS consistent, banner element exists in index.html

- [x] **T03: Improved type_text error handling to emit accessibility-missing event when CGEventTap fails due to missing permission** `est:10min`
  In the macOS type_text path, catch the CGEventTap failure from wait_for_user_click_macos and return a user-friendly error mentioning Accessibility permission. Also emit the accessibility-missing event so the banner reappears if dismissed. Ensure the error propagates cleanly to the frontend status area.
  - Files: `src-tauri/src/main.rs`
  - Verify: cargo check passes, error path returns descriptive message

## Files Likely Touched

- src-tauri/src/main.rs
- src/app.js
- src/index.html
- src/styles.css
