# S02: Core keyboard simulation via CGEvent — UAT

**Milestone:** M007
**Written:** 2026-06-24T11:30:13.803Z

## UAT: S02 Core keyboard simulation via CGEvent

### Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | cargo check passes on Windows without regressions | PASS | cargo check exit 0 in 5.16s |
| 2 | All macOS functions behind #[cfg(target_os = "macos")] | PASS | Visual inspection — cg module, post_key_event, cmd_tab, send_enter_macos, send_unicode_char_macos, enus_char_to_keycode_macos, send_vkey_char_enus_macos all have cfg guard |
| 3 | type_text macOS path replaces error stub | PASS | Error stub removed, full implementation with cmd_tab + click wait + typing loop + mode selection |
| 4 | EN-US keycode table covers same characters as Windows scancode table | PASS | Both tables cover: numbers 0-9, symbols !@#$%^&*()-_=+, letters a-z/A-Z, punctuation []{};\':\",./<>?\\|`~, space, tab |
| 5 | Memory management: all CGEvent/CGEventSource objects released | PASS | Every CGEventSourceCreate/CGEventCreateKeyboardEvent paired with CFRelease, null checks prevent use-after-create-failure |

### Verdict
**PASS** — All keyboard simulation functions implemented with correct structure. Actual macOS runtime verification deferred to macOS build.
