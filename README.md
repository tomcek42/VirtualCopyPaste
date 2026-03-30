# 🐻‍❄️ Virtual Copy Paste

A fun, modern desktop tool for typing text character-by-character into virtual machines and remote sessions where regular clipboard paste doesn't work.

Features an animated **Yeti mascot** that watches your text and covers its eyes when you mask sensitive input!

## ✨ Features

- **📋 Character-by-character paste** — Types text into the active window one keystroke at a time, bypassing VM clipboard limitations
- **🐻‍❄️ Animated Yeti** — Eyes follow your cursor as you type, mouth reacts to text length
- **🙈 Mask toggle** — Hide sensitive text and the Yeti covers its eyes with its arms
- **📌 3 clipboard slots** — Store and switch between different text snippets
- **⌨️ Global hotkeys** — `Ctrl+Shift+1/2/3` to paste slot contents from any application
- **🪶 Lightweight** — ~5MB Tauri app (vs 150MB+ Electron)

## 🚀 Getting Started

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

## 🎮 Usage

1. Launch Virtual Copy Paste
2. Type or paste your text into the input field
3. Click **📋 Paste to Target** (or press `Enter`)
4. Switch to the target window within 2 seconds
5. Text will be typed character-by-character

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+1` | Paste slot 1 content |
| `Ctrl+Shift+2` | Paste slot 2 content |
| `Ctrl+Shift+3` | Paste slot 3 content |

## 🏗️ Tech Stack

- **Frontend**: HTML/CSS/JS + [GSAP](https://gsap.com/) for SVG animation
- **Backend**: Rust + [Tauri v2](https://v2.tauri.app/)
- **Keyboard**: Windows SendInput API (Unicode)
- **Hotkeys**: tauri-plugin-global-shortcut

## 🙏 Credits

- Yeti SVG animation originally by [Darin Senneff](https://codepen.io/dsenneff) (Animated SVG Avatar)
- Adapted from William Lam's [vYetti](https://github.com/lamw/vyetti-vsphere-client-customization) project
- Original AutoIt Virtual Copy Paste concept preserved and modernized
