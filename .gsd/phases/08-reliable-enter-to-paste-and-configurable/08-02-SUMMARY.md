---
id: S02
parent: M008
milestone: M008
provides:
  - (none)
requires:
  []
affects:
  []
key_files: []
key_decisions:
  - Strg/Cmd+ENTER bleibt im Multi-Line ein fester Paste-Fallback, damit ein Enter-basierter Shortcut auch dort einen Trigger behaelt
  - Reset setzt auf 'Enter' statt zu leeren, da ein Paste-Trigger immer existieren muss
patterns_established:
  - (none)
observability_surfaces:
  - none
drill_down_paths:
  []
duration: ""
verification_result: passed
completed_at: 2026-07-02T21:08:58.798Z
blocker_discovered: false
---

# S02: Configurable Paste-to-Target shortcut in Settings (default Enter)

**Paste-to-Target-Trigger ist in den Settings per Recorder konfigurierbar (Default ENTER), persistiert und live wirksam; Multi-Line-Zeilenumbruch bleibt erhalten.**

## What Happened

T01 fuegt in den Settings ein Recorder-Feld 'Paste Shortcut' (Default Enter) hinzu, das nach dem Muster des bestehenden Activate-Hotkey-Recorders aufnimmt, in settings.json persistiert und den Wert im settings-changed-Event mitsendet; ein Reset-Button setzt auf Enter zurueck. T02 laesst das Main-Window (app.js) diesen Shortcut konsumieren: pasteShortcut wird beim Start geladen und via settings-changed live aktualisiert; neue reine Helfer parseShortcut/normalizeEventKey/eventMatchesShortcut spiegeln die Recorder-Normalisierung; handleGlobalPasteKey matcht nun den konfigurierten Shortcut statt hartkodiertem Enter. Der Multi-Line-Kontrakt bleibt: reines ENTER erzeugt einen Zeilenumbruch, Strg/Cmd+ENTER pastet als fester Fallback. Automatisch verifiziert via node --check (beide Dateien) und einem Node-Logik-Harness (9 Faelle, ALL PASS). Ein Live-Smoke-Test in tauri:dev (Shortcut aufnehmen -> im Main-Window ausloesen -> auf Enter zuruecksetzen) steht als finale Nutzerbestaetigung noch aus, analog zum S01-Vorgehen.

## Verification

node --check src/settings.js -> exit 0; node --check src/app.js -> exit 0; Node-Matcher-Harness 9/9 PASS (Default-Enter single/multi, Ctrl+Enter-Fallback, Ctrl+P single/multi, Ctrl+Enter-Shortcut). Live-Runtime-Smoke-Test in tauri:dev noch ausstehend (Nutzerbestaetigung).

## Requirements Advanced

- R001 — Paste-Trigger jetzt konfigurierbar statt hartkodiert; Default ENTER bleibt fokus-unabhaengig wirksam

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

None.

## Deviations

None.

## Known Limitations

Live-Runtime-Smoke-Test in tauri:dev (Recorder aufnehmen, im Main-Window ausloesen, Reset auf Enter) noch nicht durch den Nutzer bestaetigt. node --check und Logik-Harness decken Syntax und Matching ab, nicht das reale Tauri-Event-/UI-Verhalten.

## Follow-ups

None.

## Files Created/Modified

- `src/settings.html` — Recorder-Feld 'Paste Shortcut' + Reset-Button im Hotkey-&-Mouse-Abschnitt
- `src/settings.js` — pasteShortcut: DOM-Refs, Default Enter, Laden, Recorder-Logik, Persistenz + settings-changed-Payload, Reset auf Enter
- `src/app.js` — parseShortcut/normalizeEventKey/eventMatchesShortcut; handleGlobalPasteKey auf konfigurierbaren Shortcut umgebaut; Laden + Live-Update
