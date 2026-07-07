# Virtual Copy Paste

A lightweight desktop tool for typing text character-by-character into virtual machines and remote sessions where regular clipboard paste doesn't work.

Inspired by a 19-line [AutoIt script](Virtual%20Copy%20Paste.au3) that did exactly one thing well — and modernized into a full Tauri app with an animated Yeti mascot.

[![Download Latest Release](https://img.shields.io/github/v/release/tomcek42/VirtualCopyPaste?label=Download&style=for-the-badge)](https://github.com/tomcek42/VirtualCopyPaste/releases/latest)
![Downloads](https://img.shields.io/github/downloads/tomcek42/VirtualCopyPaste/total?style=for-the-badge&label=Downloads)

## Features

- **Character-by-character paste** — Types text into the active window one keystroke at a time, bypassing VM clipboard limitations
- **Animated Yeti** — Eyes follow your cursor as you type, covers its eyes when you mask sensitive input
- **Auto-clear timer** — Automatically clears text after paste with a countdown animation (Yeti blows it away)
- **Auto-paste clipboard on focus** — Automatically loads clipboard content when the window gains focus
- **Mask toggle** — Hide sensitive text (password mode)
- **Two keyboard modes** — Unicode (all characters) and VKey/Scancode (for nested VDI/remote sessions)
- **Target layout selection** — EN-US scancode mapping for typing on remote systems with English keyboard layouts from non-English local systems
- **Single & multi-line input** — Switch between single-line and multi-line (scripts, configs)
- **Global hotkey** — Configurable shortcut to show the window from any application
- **Optional double-click trigger** — Wait for your own double-click in the target window before typing, to select and overwrite a word
- **Settings** — General, Smart Actions (auto-clear, auto-paste), Hotkey & Mouse, Typing (keyboard mode, target layout, key-press delay), always-on-top, autostart, start minimized
- **System tray** — Close to tray, tray menu with quick access
- **Lightweight** — ~5MB Tauri app (vs 150MB+ Electron)

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (latest stable)

### Development

```bash
npm install
npx tauri dev
```

### Build

```bash
npx tauri build
```

The installer will be in `src-tauri/target/release/bundle/nsis/`.

## Usage

1. Launch Virtual Copy Paste
2. Type or paste your text into the input field
3. Click **Paste to Target** (or press `Enter`, `Ctrl+Enter` in multi-line mode)
4. Click in the target window when prompted
5. Text will be typed character-by-character

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Enter` | Paste to target (single-line mode) |
| `Ctrl+Enter` | Paste to target (multi-line mode) |
| `Escape` | Hide window to tray |
| `Ctrl+Shift+V` | Show window (default, configurable) |

## Tech Stack

- **Frontend**: HTML/CSS/JS + [GSAP](https://gsap.com/) for SVG animation
- **Backend**: Rust + [Tauri v2](https://v2.tauri.app/)
- **Keyboard**: Windows SendInput API (Unicode + VKey/Scancode)
- **Plugins**: tauri-plugin-global-shortcut, tauri-plugin-store, tauri-plugin-autostart, tauri-plugin-clipboard-manager

## Support

Virtual Copy Paste is free and, for now, Windows-only. I'm saving up for a paid
[Apple Developer Program](https://developer.apple.com/programs/) membership (99 USD/year) —
it's required for the code signing and notarization needed to ship a proper macOS build.
If this tool saves you time, a coffee helps fund the macOS port:

<a href="https://www.buymeacoffee.com/tomcek42" target="_blank">
  <img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" height="41" width="174">
</a>

## Credits

- Yeti SVG animation originally by [Darin Senneff](https://codepen.io/dsenneff) (MIT via CodePen)
- Adapted from William Lam's [vYetti](https://github.com/lamw/vyetti-vsphere-client-customization)
- Evolved from a [19-line AutoIt script](Virtual%20Copy%20Paste.au3) into a modern desktop app

## License

MIT — see [LICENSE](LICENSE).  
Third-party components (GSAP, Yeti animation): see [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).
