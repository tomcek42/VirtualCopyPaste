# M006: M006: Clipboard Auto-Paste on Window Focus

**Vision:** Eliminate the manual paste step by automatically inserting clipboard content into the single-line input when VCP gains focus, streamlining the copy-from-password-manager workflow.

## Success Criteria

- User can enable auto-paste in Settings and configure replace behavior (always replace vs. only when empty)
- Clipboard content is automatically inserted into the input field when VCP gains focus in single-line mode
- Auto-paste works regardless of how VCP gains focus (hotkey, Alt+Tab, mouse click, taskbar)
- Auto-paste does not trigger in multi-line mode

## Slices

- [x] **S01: Auto-paste settings UI and persistence** `risk:low` `depends:[]`
  > After this: User can toggle auto-paste on/off and select replace mode in Settings; values persist and are emitted to the main window via settings-changed event

- [x] **S02: Clipboard read and auto-insert on focus gain** `risk:medium` `depends:[S01]`
  > After this: When auto-paste is enabled and VCP gains focus in single-line mode, the clipboard text content is automatically inserted into the input field, respecting the configured replace mode; verified with hotkey, Alt+Tab, mouse click, and Always-on-Top scenarios

## Boundary Map

## Boundary Map

### S01 → S02

Produces:
- `autoPasteEnabled` (boolean) and `autoPasteMode` (string: "always" | "empty-only") settings keys in the store
- `settings-changed` event payload extended with `autoPasteEnabled` and `autoPasteMode` fields

Consumes:
- nothing (first slice)
