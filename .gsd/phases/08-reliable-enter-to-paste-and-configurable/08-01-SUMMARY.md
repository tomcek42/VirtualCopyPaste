---
id: S01
parent: M008
milestone: M008
provides:
  - (none)
requires:
  []
affects:
  []
key_files: []
key_decisions: []
patterns_established:
  - (none)
observability_surfaces:
  - none
drill_down_paths:
  []
duration: ""
verification_result: passed
completed_at: 2026-07-02T16:34:00.156Z
blocker_discovered: false
---

# S01: Window-global Enter triggers Paste plus auto-focus on activation

**Fenster-globaler ENTER-Handler löst Paste unabhängig vom Fokus aus; Eingabefeld wird beim Nach-vorn-Holen automatisch fokussiert.**

## What Happened

T01: `handleKeyDown` zu `handleGlobalPasteKey` umgebaut und global auf `document` registriert (app.js:169/348, Muster des ESC-Handlers). Element-gebundene keydown-Listener entfernt, sodass genau ein Paste-Pfad existiert — kein Doppel-Paste bei fokussiertem Button. ENTER pastet fokus-unabhängig, `preventDefault()` unterdrückt den zweiten Pfad. Multi-Line unverändert: reines ENTER = Zeilenumbruch, Strg+ENTER = Paste. T02: Zusätzlicher `tauri://focus`-Listener setzt `textInput.focus()` mit `isPasting`-Guard und try/catch.

## Verification

node --check src/app.js → exit 0. Manueller Smoke-Test in `npm run tauri:dev` durch den Nutzer bestätigt: App verstecken → Global-Hotkey → ENTER → Paste-Flow startet ohne Klick ins Feld. Funktioniert.

## Requirements Advanced

- R001 — Fensterweiter ENTER-Paste greift unabhängig vom Fokus

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

None.

## Follow-ups

None.

## Files Created/Modified

- `src/app.js` — Globaler keydown-Handler, entfernter element-Listener, tauri://focus Auto-Fokus
