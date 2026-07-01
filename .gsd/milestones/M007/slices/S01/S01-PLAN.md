# S01: Build and bundle config for macOS

**Goal:** Make the project compile and bundle on macOS. Add macOS Rust dependencies, update bundle config for .dmg output, add compilation stubs for all platform-specific functions, and fix the open_url fallback for macOS.
**Demo:** Run `npx tauri build` on macOS. A .dmg is produced in target/release/bundle/. The app launches and shows the Yeti UI (typing returns an error since platform code is not yet implemented).

## Must-Haves

- Complete the planned slice outcomes.

## Verification

- Run the task and slice verification checks for this slice.

## Tasks

- [x] **T01: Added core-graphics and core-foundation as macOS-conditional dependencies** `est:10min`
  Add core-graphics and core-foundation crates under [target.'cfg(target_os = "macos")'.dependencies]. These are needed for CGEvent keyboard simulation and CGEventTap mouse hooks in later slices.
  - Files: `src-tauri/Cargo.toml`
  - Verify: cargo check (or manual review that the dependency block is syntactically correct)

- [x] **T02: Updated tauri.conf.json with macOS icon paths and dmg/app bundle targets** `est:10min`
  Add .icns icon path to the icon array. Add dmg to bundle targets so macOS builds produce a .dmg. Keep nsis for Windows. Add macOS-specific bundle section if needed.
  - Files: `src-tauri/tauri.conf.json`
  - Verify: JSON is valid, icon paths exist, targets include both nsis and dmg

- [x] **T03: Fixed open_url for macOS and added macOS-specific type_text error message** `est:30min`
  Add #[cfg(target_os = "macos")] stub implementations for alt_tab, wait_for_user_click, send_unicode_char, send_vkey_char, send_vkey_char_enus, send_enter, and make_key_input. Each stub returns an error or is a no-op. Fix open_url to use 'open' on macOS instead of 'xdg-open'. Change the #[cfg(not(windows))] type_text fallback to distinguish macOS from other platforms.
  - Files: `src-tauri/src/main.rs`
  - Verify: cargo check succeeds on Windows with no regressions, cfg gates are correct for macOS

- [x] **T04: Deleted stale src-tauri/2 file (accidental npm output)** `est:1min`
  Delete the stale file src-tauri/2 which contains accidental npm install output.
  - Files: `src-tauri/2`
  - Verify: File no longer exists

## Files Likely Touched

- src-tauri/Cargo.toml
- src-tauri/tauri.conf.json
- src-tauri/src/main.rs
- src-tauri/2
