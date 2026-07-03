# M004: Hotkey & Paste Reliability

**Vision:** Make the hotkey persistent across restarts and resumes, fix Compatible-mode typing under VDI, and give the user a one-click keyboard-mode toggle in the main window.

## Success Criteria

- Custom hotkey survives app restart — console log confirms saved hotkey registers, not default
- Hotkey works within ≤2s of first press after Windows suspend/wake, no app restart needed
- Compatible mode types mixed-case text correctly into Notepad and a VDI target, including the first character, when the hotkey was pressed with Shift held
- User can flip between Unicode and Compatible mode from the main window in one click; change takes effect on the next paste without a Settings round-trip

## Slices

- [x] **S01: S01** `risk:low` `depends:[]`
  > After this: User sets a custom hotkey, saves, quits via tray, relaunches. The dev console prints `Registered activate hotkey: <custom>` (not the default), and the custom hotkey triggers the overlay on first press.

- [x] **S02: Hotkey resilience across suspend/wake** `risk:medium` `depends:[S01]`
  > After this: Rechner wird schlafen gelegt und wieder aufgeweckt. Erster Druck auf den Hotkey öffnet das Overlay ohne App-Neustart. Zusätzlich: wenn der Hotkey mid-session extern deregistriert wird, stellt der Health-Check ihn binnen ≤30s wieder her.

- [x] **S03: Compatible mode paste reliability** `risk:medium` `depends:[S01]`
  > After this: User presses the hotkey with Shift still held, pastes a mixed-case sentence ("Hello World 123") into Notepad and, where available, a VDI guest. The full text appears with correct case and no missing first character. The post-click delay value is readable from settings.json and adjustable in Settings.

- [x] **S04: In-app keyboard mode toggle** `risk:low` `depends:[S01]`
  > After this: User sees the current keyboard mode at a glance below the input. One click flips it; the label updates immediately; the next paste uses the new mode. Reopening the main window later still shows the correct state. Settings window also reflects the toggle's value.

## Boundary Map

### S01 → S02

Produces:
- A working `app.store("settings.json")` load path at `main.rs` setup time. S02's health-check reads the same hotkey value through the same accessor.

Consumes:
- nothing (first slice)

### S01 → S03

Produces:
- Confidence that store reads work at startup — S03 will read `pasteDelayMs` (new key) from the same store.

Consumes:
- nothing (first slice)

### S01 → S04

Produces:
- Same — S04 will read `keyboardMode` from the same store and write back to it live.

Consumes:
- nothing (first slice)

### S02 → S03

Produces:
- A re-register helper (likely factored from `update_hotkey`) that S03 does not depend on but that confirms the hotkey pipeline is stable before S03 touches paste internals.

Consumes:
- S01's store-load fix.

### S03 → S04

Produces:
- Stable Compatible-mode behavior, so the toggle in S04 actually surfaces a meaningful choice to the user.
- Confirmed `keyboardMode` store key shape.

Consumes:
- S01's store-load fix.
