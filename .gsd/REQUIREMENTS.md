# Requirements

This file is the explicit capability and coverage contract for the project.

## Active

## Validated

### R001 — Wenn das Hauptfenster den Fokus hat, löst der Paste-Shortcut (Default ENTER) „Paste to Target" aus, unabhängig davon, welches Element gerade den Fokus hält.
- Class: core-capability
- Status: validated
- Description: Wenn das Hauptfenster den Fokus hat, löst der Paste-Shortcut (Default ENTER) „Paste to Target" aus, unabhängig davon, welches Element gerade den Fokus hält.
- Why it matters: Nach dem Nach-vorn-Holen per Global-Hotkey liegt der Cursor oft nicht im Eingabefeld, wodurch ENTER heute wirkungslos ist. Der Paste-Trigger muss fensterweit greifen.
- Source: User request 2026-07-02: „wenn das Hauptfenster oben ist, dann soll Enter immer das Paste to Target ausführen."
- Primary owning slice: M008-S01
- Validation: Implementiert in M008/S01 (T01+T02): fenster-globaler keydown-Handler in src/app.js, genau ein Paste-Pfad. Ausgeliefert in v2.8.0 (CHANGELOG 2.8.0 "Window-global Enter-to-Paste", "No more double-paste"). UAT: .gsd/phases/08-reliable-enter-to-paste-and-configurable/08-01-UAT.md

### R002 — Der „Paste to Target"-Shortcut ist in den Settings konfigurierbar (Recorder-UI), wird in settings.json persistiert und ohne Neustart live angewendet; Default ist ENTER.
- Class: core-capability
- Status: validated
- Description: Der „Paste to Target"-Shortcut ist in den Settings konfigurierbar (Recorder-UI), wird in settings.json persistiert und ohne Neustart live angewendet; Default ist ENTER.
- Why it matters: Nutzer sollen den Paste-Trigger an ihre Gewohnheiten/Umgebungen anpassen können, ohne fest verdrahtetes ENTER.
- Source: User request 2026-07-02: „Paste to Target als Shortcut konfigurierbar machen, default sollte ENTER sein."
- Primary owning slice: M008-S02
- Validation: Implementiert in M008/S02 (T01+T02): Recorder-Feld "Paste Shortcut" in src/settings.html/.js, Persistenz in settings.json, Live-Update via settings-changed in src/app.js. Ausgeliefert in v2.8.0 (CHANGELOG 2.8.0 "Configurable Paste-to-Target shortcut"). UAT: .gsd/phases/08-reliable-enter-to-paste-and-configurable/08-02-UAT.md

### R003 — Im Multi-Line-Modus bleibt reines ENTER ein Zeilenumbruch; Paste wird dort nur durch einen Modifier-Shortcut (Default Strg+ENTER) bzw. den konfigurierten Shortcut mit Modifier ausgelöst.
- Class: constraint
- Status: validated
- Description: Im Multi-Line-Modus bleibt reines ENTER ein Zeilenumbruch; Paste wird dort nur durch einen Modifier-Shortcut (Default Strg+ENTER) bzw. den konfigurierten Shortcut mit Modifier ausgelöst.
- Why it matters: Multi-Line-Eingabe braucht ENTER für Zeilenumbrüche; ein wörtliches „ENTER pastet immer" würde die Zeilenumbruch-Eingabe unmöglich machen.
- Source: Design-Entscheidung Planung M008 (empfohlener Default, da Rückfrage ohne Antwort ablief).
- Primary owning slice: M008-S02
- Validation: Implementiert in M008/S02 (T02): Multi-Line-Zweig in handleGlobalPasteKey (src/app.js) laesst reines ENTER als Zeilenumbruch durch; Paste nur ueber konfigurierten Modifier-Shortcut bzw. Strg/Cmd+ENTER-Fallback. Ausgeliefert in v2.8.0 (CHANGELOG 2.8.0 "Changed: Multi-line mode"). UAT: .gsd/phases/08-reliable-enter-to-paste-and-configurable/08-02-UAT.md

## Deferred

## Out of Scope

## Traceability

| ID | Class | Status | Primary owner | Supporting | Proof |
| --- | --- | --- | --- | --- | --- |
| R001 | core-capability | validated | M008-S01 | none | Implementiert in M008/S01 (T01+T02): fenster-globaler keydown-Handler in src/app.js, genau ein Paste-Pfad. Ausgeliefert in v2.8.0 (CHANGELOG 2.8.0 "Window-global Enter-to-Paste", "No more double-paste"). UAT: .gsd/phases/08-reliable-enter-to-paste-and-configurable/08-01-UAT.md |
| R002 | core-capability | validated | M008-S02 | none | Implementiert in M008/S02 (T01+T02): Recorder-Feld "Paste Shortcut" in src/settings.html/.js, Persistenz in settings.json, Live-Update via settings-changed in src/app.js. Ausgeliefert in v2.8.0 (CHANGELOG 2.8.0 "Configurable Paste-to-Target shortcut"). UAT: .gsd/phases/08-reliable-enter-to-paste-and-configurable/08-02-UAT.md |
| R003 | constraint | validated | M008-S02 | none | Implementiert in M008/S02 (T02): Multi-Line-Zweig in handleGlobalPasteKey (src/app.js) laesst reines ENTER als Zeilenumbruch durch; Paste nur ueber konfigurierten Modifier-Shortcut bzw. Strg/Cmd+ENTER-Fallback. Ausgeliefert in v2.8.0 (CHANGELOG 2.8.0 "Changed: Multi-line mode"). UAT: .gsd/phases/08-reliable-enter-to-paste-and-configurable/08-02-UAT.md |

## Coverage Summary

- Active requirements: 0
- Mapped to slices: 0
- Validated: 3 (R001, R002, R003)
- Unmapped active requirements: 0
