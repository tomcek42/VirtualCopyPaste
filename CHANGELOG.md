# Changelog

## 2.5.4 - 2026-05-16

### Fixed

- Update checker false positive: added semver comparison so the app no longer shows "Update available" when the installed version matches or exceeds the remote version.

## 2.5.3 - 2026-05-16

### Fixed

- Update checker falsely reporting available updates (Tauri v2 API compatibility).
- PowerShell window flashing when clicking links in the About window (replaced `cmd /C start` with `ShellExecuteW`).
- Shift/modifier key loss in Compatible Keyboard Mode over nested remote sessions (RDP → VMware console). Modifier events are now sent individually with configurable intra-key delay.

### Added

- "Key Press Delay" setting in Keyboard tab for tuning modifier timing in high-latency nested sessions (default 5ms, max 50ms).
- Proxy support: improved error messages with `HTTPS_PROXY` hint when update check fails behind a corporate proxy.

## 2.5.2 - 2026-05-05

### Fixed

- Settings window opening as blank/unresponsive white window. Root cause: synchronous IPC command deadlocked the main thread during WebView2 initialization. Fixed by making the `open_settings` command async with `run_on_main_thread` dispatch.

### Changed

- Update notice banner now correctly adjusts main window height.
- Frontend-side update check as fallback when backend event is missed.

## 2.5.1 - 2026-05-04

### Changed

- Bundle identifier changed to `VirtualCopyPaste`. Settings are now stored under `%APPDATA%\VirtualCopyPaste\`.

## 2.5.0 - 2026-05-04

### Added

- Settings sidebar navigation: categories (General, Hotkeys, Keyboard, Updates) replace the single scrollable list.
- "Check for Updates" button in Settings with consent-based update flow — no more automatic download and restart.
- "Automatically check for updates on startup" toggle in Settings.
- Update notification banner in main window when a new version is available.
- "Start Minimized" setting now persisted in store (no longer requires `--start-minimized` CLI flag).

### Changed

- Settings window layout: two-column sidebar + content panel (480×450px) replaces vertical list (380×680px).
- Update flow requires user confirmation before downloading and installing — no forced restart.
- Backend update check only emits an event; download happens only after user clicks "Install & Restart".

## 2.4.1 - 2026-05-03

### Changed

- CI workflow updated to Node.js 24.
- Updater artifacts switched from v1Compatible to v2 format.

### Fixed

- Cargo.toml version synced (was stuck at 2.3.0).

## 2.4.0 - 2026-05-03

### Added

- Auto-updater with signed NSIS updates via GitHub Releases (tauri-plugin-updater).
- GitHub Actions release workflow: builds, signs, and auto-publishes on version tags.
- Changelog-based release notes extracted automatically from CHANGELOG.md.

## 2.3.0 - 2026-05-02

### Added

- Keyboard mode toggle switch (Std/Compat) in the main window below the Paste button, with bidirectional sync to the Settings window.
- Status bar feedback when switching keyboard modes.

### Changed

- Default hotkey changed from `Ctrl+Shift+Space` to `Ctrl+Shift+V`.
- Installer now offers per-user or per-machine install (was per-user only).
- Main window starts hidden to prevent brief flash on startup; shown explicitly when not launched minimized.
- Main window height increased from 255 to 270 for toggle spacing.
- About window height increased from 480 to 500.

### Fixed

- Hotkey loaded from disk at startup instead of always falling back to default ([a23cb54]).
- Window no longer briefly flashes visible before hiding when started minimized.

## 2.2.0 - 2026-04-08

### Added

- ESC key hides window to system tray.
- Clear button (x) in text field, visible only when text is present.
- About window with version info, keyboard mode comparison table, and credits.
- GitHub link opens browser from About window.
- Version display in tray menu.
- About entry in tray menu.
- Latest release download badge in README.

### Changed

- Unified version to 2.2.0 across package.json, Cargo.toml, tauri.conf.json.
- Cleaned up README, removed loose SVG/PNG files from root.
- About window shows app icon, clickable credit links (Darin Senneff, William Lam vYetti).

## 2.1.0 - 2026-03-30

### Added

- Smart paste: saves cursor position, Alt+Tab to target, clicks at saved position, then types. Ensures focus in target apps including console windows.
- SVG icons replace all emoji icons (eye.svg, eye-off.svg).
- Custom app icon (textbox.svg converted to icon.ico at 16/32/48/256px).
- Cargo release profile (strip, LTO, codegen-units=1, opt-level=s) — release exe 4.3 MB, NSIS installer 1.4 MB.

### Fixed

- Yeti animation: removed mouth morph logic (mouth stays closed like original), deleted unused MorphSVGPlugin.min.js.

[a23cb54]: https://github.com/tomcek42/VirtualCopyPaste/commit/a23cb54
