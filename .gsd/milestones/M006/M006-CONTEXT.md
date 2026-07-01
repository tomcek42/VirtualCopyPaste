# M006: Clipboard Auto-Paste on Window Focus

**Gathered:** 2026-05-31
**Status:** Ready for planning

## Project Description

Automatically paste the system clipboard content into the single-line input field when the Virtual Copy Paste window gains focus. This eliminates the manual Ctrl+V step in the common workflow: copy from password manager → bring VCP to foreground → paste → send to target.

## Why This Milestone

The most common VCP workflow involves copying text (often a password) from another application, switching to VCP, manually pasting it, then sending it to the target VM/session. The manual paste step is redundant friction — the user already copied the text specifically to send it through VCP. Auto-pasting on focus eliminates this step.

## User-Visible Outcome

### When this milestone is complete, the user can:

- Enable "Auto-paste clipboard on focus" in Settings
- Copy text in any app (e.g. 1Password), switch to VCP (hotkey, Alt+Tab, or mouse click), and see the clipboard content already in the input field
- Configure whether auto-paste only fills an empty field or always replaces existing text (default: always replace)

### Entry point / environment

- Entry point: Main window (single-line input mode) + Settings window
- Environment: Windows desktop (Tauri app)
- Live dependencies involved: Windows clipboard API (via Tauri or JS navigator.clipboard)

## Completion Class

- Contract complete means: settings persist, clipboard is read and inserted on focus gain
- Integration complete means: auto-paste interacts correctly with mask toggle, Always-on-Top focus cycling, mode switching, and auto-clear timer (M005)
- Operational complete means: none

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- User enables auto-paste, copies text in another app, brings VCP to foreground → text appears in input field
- With "always replace" mode: existing text is replaced by clipboard content
- With "only when empty" mode: existing text is preserved, clipboard is only pasted into empty field
- Switching to multi-line mode disables auto-paste behavior
- Auto-paste works regardless of how VCP gains focus (hotkey, Alt+Tab, mouse click, taskbar)

## Architectural Decisions

### Use window focus event for trigger

**Decision:** Listen to the Tauri window `focus` event (or JS `window.focus`) to detect when VCP comes to the foreground.

**Rationale:** Works consistently regardless of how the window is activated (hotkey, mouse, Alt+Tab, taskbar click). Even with Always-on-Top enabled, focus events still fire when the user clicks in/out of the VCP window.

**Alternatives Considered:**
- Hotkey-only trigger — would not work when Always-on-Top is active and user doesn't need the hotkey
- Continuous clipboard polling — wasteful, privacy concern, and would paste even when user doesn't want it

### Clipboard read via JavaScript navigator.clipboard API

**Decision:** Use the `navigator.clipboard.readText()` API from the webview context.

**Rationale:** Simplest approach, no Rust-side clipboard dependency needed. Tauri v2 webviews support the Clipboard API. Falls back gracefully if clipboard access is denied.

**Alternatives Considered:**
- Rust-side clipboard crate — adds a dependency, requires IPC round-trip, more complex
- Tauri clipboard plugin — may not be needed if the JS API works in the webview context

### Configurable replace behavior

**Decision:** Two modes: "always replace" (default) and "only when empty". Configurable in Settings.

**Rationale:** User requested "always replace" as default since the typical workflow is: copy → switch to VCP → send. But "only when empty" provides safety for users who sometimes type text manually and don't want it overwritten.

## Error Handling Strategy

If clipboard read fails (permission denied, empty clipboard, non-text content), silently do nothing — no error message, no field change. The user can always manually paste.

## Risks and Unknowns

- Clipboard API availability in Tauri v2 webview — low risk, modern Chromium-based webview should support it
- Focus event reliability with Always-on-Top — medium risk, needs testing to confirm focus events fire correctly when clicking in/out of an always-on-top window

## Existing Codebase / Prior Art

- `src/app.js` — main app controller, handles input mode and focus management
- `src/settings.js` — settings controller, will add new auto-paste options
- `src/settings.html` — settings UI, will add new section
- `src-tauri/src/main.rs` — Rust backend, may need a Tauri command for clipboard read if JS API doesn't work

## Relevant Requirements

- No formal requirements registered yet

## Scope

### In Scope

- Settings: enable/disable auto-paste, replace mode (always / only when empty)
- Clipboard read on window focus gain
- Auto-paste only in single-line mode
- Interaction with Always-on-Top focus behavior

### Out of Scope / Non-Goals

- Auto-paste in multi-line mode
- Clipboard history or multi-item clipboard
- Clipboard monitoring / background polling
- Auto-pasting images or rich text (text only)

## Technical Constraints

- Must work with the existing Always-on-Top feature
- Must not read clipboard in background (only on explicit focus gain)
- Must handle clipboard permission gracefully

## Integration Points

- `settings-changed` event — new fields for auto-paste settings
- Window focus event — trigger for clipboard read
- M005 auto-clear timer — if both are enabled, auto-paste should reset the auto-clear timer

## Testing Requirements

Manual testing in the running app:
- Enable auto-paste, copy text, switch to VCP → text appears
- With "always replace": type text, switch away, copy new text, switch back → new text replaces old
- With "only when empty": type text, switch away, copy new text, switch back → old text preserved
- Switch to multi-line mode → auto-paste does not trigger
- Empty clipboard → no change to input field
- With Always-on-Top: click out of VCP, click back → auto-paste triggers

## Acceptance Criteria

- S01: Settings UI shows auto-paste toggle and replace mode selector, values persist
- S02: Clipboard content is read and inserted into the input field on focus gain in single-line mode, respecting the replace mode setting

## Open Questions

- Does `navigator.clipboard.readText()` work reliably in Tauri v2 webviews on Windows? If not, a Rust-side fallback via clipboard crate will be needed.
