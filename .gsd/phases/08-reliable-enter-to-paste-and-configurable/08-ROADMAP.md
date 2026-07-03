# M008: Reliable Enter-to-Paste and Configurable Paste Shortcut

**Vision:** Wer das Fenster per Global-Hotkey nach vorn holt, kann sofort ENTER drücken und in das Zielfenster pasten — ohne vorher ins Eingabefeld zu klicken. Der Paste-Trigger ist zudem in den Settings frei konfigurierbar (Default ENTER), wird persistiert und ohne Neustart live wirksam. Multi-Line-Eingabe mit ENTER-Zeilenumbrüchen bleibt erhalten.

## Success Criteria

- Nach Global-Hotkey-Aktivierung löst ein einzelner ENTER-Druck Paste-to-Target aus, auch wenn der Fokus nicht im Eingabefeld liegt.
- Der Paste-Shortcut ist in den Settings per Recorder konfigurierbar, Default ENTER, wird in settings.json persistiert und ohne Neustart übernommen.
- Im Multi-Line-Modus fügt reines ENTER weiterhin einen Zeilenumbruch ein; Paste erfolgt über Modifier-Shortcut (Default Strg+ENTER).
- Kein Doppel-Paste, wenn der Paste-Button den Fokus hat.

## Slices

- [x] **S01: Window-global Enter triggers Paste plus auto-focus on activation** `risk:medium` `depends:[]`
  > After this: App verstecken, per Global-Hotkey nach vorn holen ohne ins Feld zu klicken, ENTER drücken → Alt+Tab/Paste-Flow startet. Im Multi-Line-Modus fügt reines ENTER weiterhin einen Zeilenumbruch ein, Strg+ENTER pastet.

- [x] **S02: Configurable Paste-to-Target shortcut in Settings (default Enter)** `risk:medium` `depends:[S01]`
  > After this: Settings öffnen, Paste-Shortcut Strg+P aufnehmen, speichern; im Main-Window Strg+P → Paste startet; auf ENTER zurücksetzen → ENTER pastet; reines ENTER im Multi-Line erzeugt weiter einen Zeilenumbruch.

## Boundary Map

Not provided.
