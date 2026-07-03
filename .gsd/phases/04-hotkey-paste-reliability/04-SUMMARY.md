---
id: M004
title: "Hotkey & Paste Reliability"
status: complete
completed_at: 2026-06-24T06:28:36.872Z
key_decisions:
  - EN-US Scancode-Tabelle statt VkKeyScan für Compatible-Mode
  - User-driven Double-Click statt simuliertem Klick (v2.7.4)
  - Hotkey aus Store laden beim Start statt Default zu registrieren (S01)
key_files:
  - src-tauri/src/main.rs
  - src/app.js
  - src/settings.js
  - src/settings.html
  - src/index.html
lessons_learned:
  - (none)
---

# M004: Hotkey & Paste Reliability

**Hotkey überlebt Neustarts und Suspend/Wake zuverlässig, Compatible-Mode tippt Mixed-Case korrekt, und der Keyboard-Mode ist per One-Click-Toggle im Hauptfenster umschaltbar.**

## What Happened

M004 adressierte vier Bereiche der Hotkey- und Paste-Zuverlässigkeit:

S01 (complete): Hotkey-Persistenz nach Neustart — Custom-Hotkey wird aus dem Store geladen und beim Start registriert statt auf den Default zurückzufallen.

S02 (skipped): Hotkey-Resilienz nach Suspend/Wake — vom User als bereits funktionsfähig bestätigt, kein zusätzlicher Code nötig.

S03 (skipped): Compatible-Mode Paste-Zuverlässigkeit — EN-US Scancode-Mapping mit korrektem Shift-Handling war bereits in main.rs implementiert.

S04 (skipped): In-App Keyboard-Mode Toggle — Std/Compat-Toggle im Hauptfenster mit Store-Persistenz und bidirektionalem Settings-Sync war bereits implementiert.

Die meisten Features wurden in v2.6.0–v2.7.4 bereits umgesetzt, bevor M004 formal geplant wurde.

## Success Criteria Results

Alle vier Success Criteria erfüllt: (1) Custom-Hotkey überlebt Neustart, (2) Hotkey funktioniert nach Suspend/Wake, (3) Compatible-Mode tippt Mixed-Case korrekt, (4) One-Click Keyboard-Mode Toggle im Hauptfenster vorhanden.

## Definition of Done Results

Not provided.

## Requirement Outcomes

Not provided.

## Deviations

S02, S03, S04 wurden übersprungen da die Features bereits vor der formalen Planung implementiert waren.

## Follow-ups

None.
