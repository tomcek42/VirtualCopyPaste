---
id: M008
title: "Reliable Enter-to-Paste and Configurable Paste Shortcut"
status: complete
completed_at: 2026-07-03T05:17:18.755Z
key_decisions: []
key_files: []
lessons_learned:
  - (none)
---

# M008: Reliable Enter-to-Paste and Configurable Paste Shortcut

**Fokus-unabhängiges Enter-to-Paste plus frei konfigurierbarer, live wirksamer Paste-Shortcut in den Settings (Default ENTER).**

## What Happened

M008 liefert zuverlässiges fokus-unabhängiges Enter-to-Paste (S01) und einen frei konfigurierbaren Paste-Shortcut in den Settings mit Default ENTER, Persistenz und Live-Update ohne Neustart (S02). Der Multi-Line-Kontrakt bleibt erhalten (reines ENTER = Zeilenumbruch, Strg+ENTER = Paste). Zusätzlich behoben: Modifier-basierte Shortcuts (z. B. Ctrl+P) lösten durch gehaltene Modifier + simuliertes Alt+Tab die Windows-Task-View aus; `alt_tab()` gibt gehaltene Modifier nun im selben SendInput-Batch frei. UI-Feinschliff: Abschnitt „Hotkeys & Mouse" (einzeilig), Reset-Button für Activate Hotkey (Default Ctrl+Shift+V), überflüssiger Clear-Button entfernt. Verifiziert via node --check, 9/9-Matcher-Harness und Nutzer-Live-Tests. Version auf 2.8.0 gebumpt.

## Success Criteria Results

Not provided.

## Definition of Done Results

Not provided.

## Requirement Outcomes

Not provided.

## Deviations

None.

## Follow-ups

None.
