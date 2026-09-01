# Manual paste regression test

`paste-test.txt` is a fixture for the keyboard simulation path. Open it in an editor,
copy everything, paste it into Virtual Copy Paste (Multi-Line mode), then paste into the
target window and diff the result against the file.

21 lines, 648 characters — roughly 15 seconds at the default 20 ms typing delay.

Run it four times: **Standard** and **Compatible** keyboard mode, each with
*Strip Auto-Indent After Line Break* on and off.

Use two targets. Windows Editor never auto-indents, so it can only prove character
fidelity. Notepad++ does auto-indent and is the only target that can prove the
auto-indent handling — but disable its word completion first, see below.

## What each line covers

| Line | Covers |
|---|---|
| 02–04 | **Dead keys.** `` ` ``, `´`, `^` must survive on their own. Line 03 is the combining test: `^a` must not become `â`. |
| 05 | **AltGr characters.** The vkey path sends a real right Alt (`VK_RMENU` + `KEYEVENTF_EXTENDEDKEY`), not Ctrl+left Alt. Before that change `}` `²` `³` were dropped even locally. `€` is not on `VkKeyScanW` for de-DE and still falls back to Unicode, so it is expected to fail in a nested remote session. |
| 06 | Full shift row, including `"` and `'`, which are dead keys on some layouts. |
| 07 | Umlauts plus `é à ñ` — `VkKeyScanW` cannot map those on a German layout, so this exercises the Unicode fallback inside Compatible mode. |
| 08 | Doubled backslash (UNC path) and `$`. |
| 10–15 | **Tab indentation plus the auto-indent trap.** The open braces make editors auto-indent; this is where the target's indentation and the text's own indentation would add up. Line 12 also has backticks inside indented text. |
| 17–18 | Space indentation — must not collide with the tabs. |
| 20 | Empty line — two Enters with nothing between them. |
| 21 | **Dead key as the very last character of the text.** If it shows up, the dead-key flush is airtight. |

## Reading the result

The line numbers in the text are the real line numbers, so `21` must end up on line 21.
A mismatch between the printed number and the editor's line number pinpoints where a line
was lost or added. Lines 10–15 deliberately carry no number so their indentation stays
unaltered.

## Known-bad targets

These are target behaviours a keyboard simulation cannot win against. Rule them out
before filing a bug.

**Notepad++ — word completion eats the Enter.** With
`Settings > Auto-Completion > Function and word completion` enabled and
*Insert selection with ENTER* on (`autoCAction="3"`, `insertSelectedItemUseENTER="yes"`
in `config.xml`), Enter is consumed by the completion popup whenever the line ends on a
word that already exists in the document. In this fixture that is exactly line 18
(`Text`, first seen on line 17) — the only such line, and reproducibly the only defect.
The damage is amplified by *Strip Auto-Indent*: the Shift+Home cleanup then runs against
the previous line and its text is overwritten by the next character. Turn word completion
off for the test run.

**VS Code — not a usable target.** Auto-closing brackets add stray `}` at the end,
IntelliSense swallows Enter on random lines, and Tab is an indent command while a
selection is standing, so the Shift+Home cleanup costs one tab level per line. None of
this is reproducible or fixable from the sending side.
