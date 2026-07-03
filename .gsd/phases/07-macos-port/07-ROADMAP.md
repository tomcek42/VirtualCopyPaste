# M007: macOS Port

**Vision:** Port Virtual Copy Paste to macOS so it builds, installs, and runs natively on Apple Silicon and Intel Macs. The frontend (Yeti animation, UI) stays unchanged. The Rust backend gets macOS-native implementations for keyboard simulation (CGEvent), mouse click detection (CGEventTap), window switching (Cmd+Tab), and platform utilities. The app requests Accessibility permission on first launch and guides the user through granting it.

## Success Criteria

- App builds as .dmg on macOS via `npx tauri build` without errors
- Unicode mode types mixed-case text correctly into TextEdit and Terminal on macOS
- Compatible mode types EN-US mapped characters correctly on macOS
- Cmd+Tab switches to previous window, click detection works, then typing starts
- Double-click mode works on macOS (waits for 2 clicks before typing)
- Accessibility permission prompt appears on first launch if not granted; app detects when granted
- CI/CD produces both Windows (.exe) and macOS (.dmg) artifacts on tag push
- All existing Windows functionality remains unchanged

## Slices

- [x] **S01: Build and bundle config for macOS** `risk:low` `depends:[]`
  > After this: Run `npx tauri build` on macOS. A .dmg is produced in target/release/bundle/. The app launches and shows the Yeti UI (typing returns an error since platform code is not yet implemented).

- [x] **S02: Core keyboard simulation via CGEvent** `risk:high` `depends:[S01]`
  > After this: User pastes text in the app, it Cmd+Tabs to the previous window, waits for a click, then types 'Hello World 123!\nLine 2' correctly into TextEdit. Both Standard (Unicode) and Compatible modes work.

- [x] **S03: Mouse click detection via CGEventTap** `risk:medium` `depends:[S01]`
  > After this: After Cmd+Tab, the app waits for the user to click (or double-click) in the target window. The click reaches the target app, then typing begins.

- [x] **S04: Accessibility permission check and user guidance** `risk:medium` `depends:[S02,S03]`
  > After this: Fresh install on macOS: app launches, detects missing Accessibility permission, shows a non-blocking banner or dialog explaining how to grant it. After user grants permission in System Settings, the next paste attempt works without app restart.

- [x] **S05: CI and CD multi-platform build** `risk:low` `depends:[S01]`
  > After this: Push a version tag. GitHub Actions builds both Windows NSIS installer and macOS .dmg. Both appear as release assets.

## Boundary Map

### S01 -> S02, S03, S05\n\nProduces:\n- Cargo.toml with macOS dependencies (core-graphics, core-foundation)\n- Compilable project on macOS\n- Bundle config for .dmg output\n\nConsumes:\n- nothing (first slice)\n\n### S02 -> S04\n\nProduces:\n- Working keyboard simulation (send_unicode_char, send_vkey_char, send_enter, alt_tab for macOS)\n- Verified that CGEvent APIs produce correct output\n\nConsumes:\n- S01 macOS build scaffold and dependencies\n\n### S03 -> S04\n\nProduces:\n- Working click detection (wait_for_user_click via CGEventTap)\n- Verified that clicks pass through to target app\n\nConsumes:\n- S01 macOS build scaffold and dependencies\n\n### S02 + S03 -> S04\n\nS04 (Accessibility permission) depends on both S02 and S03 being functional so it can test the full flow: if permission is missing, typing and click detection fail gracefully with a user-facing message.\n\n### S01 -> S05\n\nS05 (CI and CD) only needs the build config from S01. It can run in parallel with S02-S04.
