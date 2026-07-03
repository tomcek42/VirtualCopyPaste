# Virtual Copy Paste

A modern Windows desktop tool for typing text character-by-character into virtual machines and remote sessions where clipboard paste doesn't work.

**Current version: v2.8.0** — Windows only.

## What It Does

- **Character-by-character paste**: Types text into the active window one character at a time, bypassing VM clipboard limitations
- **Animated Yeti mascot**: An SVG Yeti character watches your text input and covers its eyes when you mask sensitive text
- **Multi-slot clipboard**: 3 independent text slots for storing different snippets
- **Global hotkeys**: Ctrl+Shift+1/2/3 to paste slot contents from any application
- **Mask toggle**: Hide sensitive input with password masking — the Yeti covers its eyes!
- **Enter-to-Paste**: Press Enter in the text area to paste directly (configurable shortcut)
- **Configurable paste shortcut**: Choose between Enter, Ctrl+Enter, or Shift+Enter
- **Auto-clear timer**: Yeti countdown animation before clearing sensitive text
- **Clipboard auto-paste**: Automatically pastes clipboard content on window focus
- **Auto-updater**: Built-in update check with proxy support

## Tech Stack

- **Frontend**: HTML/CSS/JS with GSAP v3 for SVG animations
- **Backend**: Rust with Tauri v2
- **Keyboard simulation**: Windows SendInput API (KEYEVENTF_UNICODE)
- **Global hotkeys**: tauri-plugin-global-shortcut
- **Platform**: Windows only (macOS port exists on `macos-port` branch, not released)

## Running

```bash
npm install
npx tauri dev
```

## Building

```bash
npx tauri build
```

Produces an `.exe` installer via NSIS in `src-tauri/target/release/bundle/`.

## Release

Tags on `master` trigger CI (GitHub Actions) which builds Windows-only NSIS installer and publishes a GitHub Release.

## Credits

- Yeti SVG animation by Darin Senneff (Animated SVG Avatar)
- Adapted from William Lam's [vYetti](https://github.com/lamw/vyetti-vsphere-client-customization) project
- Original AutoIt script concept preserved and modernized
