# Virtual Copy Paste

A modern desktop tool for typing text character-by-character into virtual machines and remote sessions where clipboard paste doesn't work.

## What It Does

- **Character-by-character paste**: Types text into the active window one character at a time, bypassing VM clipboard limitations
- **Animated Yeti mascot**: An SVG Yeti character watches your text input and covers its eyes when you mask sensitive text
- **Multi-slot clipboard**: 3 independent text slots for storing different snippets
- **Global hotkeys**: Ctrl+Shift+1/2/3 to paste slot contents from any application
- **Mask toggle**: Hide sensitive input with password masking — the Yeti covers its eyes!

## Tech Stack

- **Frontend**: HTML/CSS/JS with GSAP v3 for SVG animations
- **Backend**: Rust with Tauri v2
- **Keyboard simulation**: Windows SendInput API (KEYEVENTF_UNICODE)
- **Global hotkeys**: tauri-plugin-global-shortcut

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

## Credits

- Yeti SVG animation by Darin Senneff (Animated SVG Avatar)
- Adapted from William Lam's [vYetti](https://github.com/lamw/vyetti-vsphere-client-customization) project
- Original AutoIt script concept preserved and modernized
