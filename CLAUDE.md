# Virtual Copy Paste

Tauri v2 app that types clipboard text into another window by simulating keystrokes.
Target environment is nested remote sessions (Horizon VDI → vCenter Console → VM), where
the clipboard does not pass through.

## Platform

**Windows only.** The macOS port lives on the `macos-port` branch and is parked (keyboard
simulation there only ever typed `a`). Do not merge macOS code into `master` — the release
workflow is Windows-only on purpose, and a tag on master with macOS code would publish
broken artifacts.

## Keyboard simulation — the parts that were paid for in blood

All of this is in `src-tauri/src/main.rs`. Every rule below is a fix for a defect that was
measured, not a precaution. Changing any of them without a regression run will reintroduce
the defect.

- **AltGr is `VK_RMENU` + `KEYEVENTF_EXTENDEDKEY`, never `VK_CONTROL` + `VK_LMENU`.**
  `VkKeyScanW` reports AltGr as shift state 6 (Ctrl+Alt), and sending it literally as two
  modifiers is what the layout would accept — but Ctrl+Alt+key is a hotkey combination
  that shells, editors and remote clients intercept before the layout sees it. Locally
  that silently dropped `}` `²` `³` while `{ [ ] \ | ~ @ µ` came through; the mapping was
  provably correct via `ToUnicodeEx`, only the delivery failed.

- **Auto-indent cleanup selects, it never deletes.** `send_enter()` sends Shift+Home and
  leaves the selection standing so the next character overwrites it. An earlier version
  sent Delete, which on a target that did *not* auto-indent ate the character *after* the
  cursor. Selection-only is harmless in that case because the selection is empty.

- **Dead keys must be probed and flushed.** `` ` ´ ^ ~ `` arm an accent instead of emitting
  a character. `is_dead_key()` asks `ToUnicodeEx` with wFlags bit 2 (no layout state
  mutation), and `flush_dead_key()` presses Space to commit the accent. Without it a dead
  key at the end of the text never appears at all.

- **Tab is a real Tab keypress, never `KEYEVENTF_UNICODE`.** As U+0009 it arrives as a
  `WM_CHAR` that edit controls, browsers and most editors silently drop.

- **`VkKeyScanW` returning -1 falls back to Unicode silently.** On de-DE that hits `€`
  and `é à ñ`. Measured in a nested Horizon → vCenter → VM session (04.09.2026), only
  `é à ñ` actually drop out there — `€` survives the Unicode path. It works locally either way. The fix is the Target Layout setting:
  `dede_char_to_scancode()` next to `enus_char_to_scancode()`, reached via Compatible mode
  + Target Layout DE-DE.

- **The Target Layout tables describe the *target*, not the local machine.** Both
  `enus_char_to_scancode()` and `dede_char_to_scancode()` send bare scancodes with no
  virtual key, so the target resolves them through its own layout and the local layout is
  irrelevant. Do not "fix" them with `MapVirtualKeyW` — that reintroduces the local
  layout and is exactly what these paths exist to bypass.

- **DE-DE dead keys are a hardcoded set, not a `ToUnicodeEx` probe.** `dede_is_dead()`
  lists `^` and the two accents on scancode 0x0D. `is_dead_key()` cannot be reused here
  because it queries the *local* layout, and this path exists precisely for the case where
  local and target differ. `é à û` are composed as dead key + base letter via
  `dede_dead_compose()`; `ñ ã õ ç` stay Unicode because German T1 has no dead tilde and no
  cedilla. Note QWERTZ when editing the tables: `y` is scancode 0x2C, `z` is 0x15, and 0x56
  (`< > |`) does not exist on EN-US at all.

- **`send_enter()` is shared by all three send paths** (Unicode, vkey, EN-US vkey).
  Standard and Compatible mode behave identically for line breaks. Do not tie the
  Strip-Auto-Indent option to the keyboard mode — it depends on the *target*, not the mode.

- **Known residual damage:** if a target swallows an Enter (autocomplete popup, IntelliSense),
  the Shift+Home cleanup then runs against the *previous* line and its text gets
  overwritten. A lost line break becomes a lost line. Not fixable without feedback from the
  target — `Escape` before every Enter would be fatal in `vi` over SSH. This is why the
  Strip-Auto-Indent checkbox still exists.

## Testing

The keyboard path cannot be tested automatically — it depends on how a foreign window
reacts to injected input. `tests/README.md` has the manual regression procedure and the
list of known-bad targets (Notepad++ word completion, VS Code). Read it before concluding
that a defect is ours.

Notepad++ is the only proven target for the auto-indent behaviour; Windows Editor never
auto-indents and therefore cannot prove it. Test output files (`tests/Test_*.txt`) are
gitignored local evidence.

## Release

Bump the version in **all three** files or the build produces mismatched artifacts:
`package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`.

Run `npm run tauri:build` locally before tagging a release — CI failures on a tag are
expensive to unwind.

## Conventions

- `.gsd/` is GSD workflow state and is tracked on purpose (except the transient files
  listed in `.gitignore`). Never touch `.gsd/gsd.db` with `sqlite3` — use the GSD tools.
- The repository uses CRLF in the working tree. `git diff` showing a whole file changed
  usually means only line endings differ.
