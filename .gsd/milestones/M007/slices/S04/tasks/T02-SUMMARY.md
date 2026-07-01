---
id: T02
parent: S04
milestone: M007
key_files:
  - src-tauri/src/main.rs
  - src/app.js
  - src/index.html
  - src/styles.css
key_decisions:
  - (none)
duration: 
verification_result: passed
completed_at: 2026-06-24T11:38:40.351Z
blocker_discovered: false
---

# T02: Added startup accessibility check, frontend banner with Grant Access button, and auto-dismiss on focus when permission granted

**Added startup accessibility check, frontend banner with Grant Access button, and auto-dismiss on focus when permission granted**

## What Happened

In Tauri setup hook (macOS only), emit accessibility-missing event if AXIsProcessTrusted returns false. Frontend listens for this event and shows a red-tinted banner above the update notice. Banner has a Grant Access button that calls open_accessibility_settings. On window focus, if banner is visible, re-checks permission via check_accessibility_permission and hides banner if granted. Window resize logic accounts for both banners.

## Verification

cargo check passed, HTML/CSS/JS consistent

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo check --manifest-path src-tauri/Cargo.toml` | 0 | pass | 3470ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `src-tauri/src/main.rs`
- `src/app.js`
- `src/index.html`
- `src/styles.css`
