# S01: Build and bundle config for macOS — UAT

**Milestone:** M007
**Written:** 2026-06-24T06:45:09.661Z

## UAT: S01 — Build and bundle config for macOS

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Cargo.toml has macOS-conditional dependencies | PASS | core-graphics 0.24 and core-foundation 0.10 under [target.'cfg(target_os = "macos")'.dependencies] |
| 2 | tauri.conf.json includes .icns icon and dmg target | PASS | icons array has icon.icns, targets: ["nsis", "dmg", "app"] |
| 3 | open_url uses 'open' on macOS | PASS | #[cfg(target_os = "macos")] block with Command::new("open") |
| 4 | type_text returns macOS-specific error | PASS | #[cfg(target_os = "macos")] returns "macOS keyboard simulation not yet implemented" |
| 5 | cargo check passes on Windows (no regression) | PASS | exit 0, 22s |
| 6 | Stale src-tauri/2 file removed | PASS | File deleted |

**Verdict:** PASS — all config changes in place, Windows build unaffected. macOS build verification deferred to a machine with macOS.
