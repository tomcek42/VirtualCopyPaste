# Manual paste regression test

`paste-test.txt` is a fixture for the keyboard simulation path. Open it in an editor,
copy everything, paste it into Virtual Copy Paste (Multi-Line mode), then paste into the
target window and diff the result against the file.

21 lines, 648 characters — roughly 15 seconds at the default 20 ms typing delay.

## The runs

The matrix splits in two, because the two axes prove different things. Auto-indent
handling depends only on the target window — `send_enter()` is identical for all three
send paths — so it can be proven locally. Character fidelity over a nested session can
only be proven in the session.

| # | Where | Target | Keyboard Mode | Target Layout | Strip Auto-Indent | Proves |
|---|---|---|---|---|---|---|
| 1 | local | Notepad++ | Standard | – | on | auto-indent cleanup (regression) |
| 2 | local | Notepad++ | Standard | – | off | that Notepad++ indents at all (control run) |
| 3 | local | Notepad++ | Compatible | DE-DE | on | DE-DE table + cleanup |
| 4 | local | Notepad++ | Compatible | DE-DE | off | DE-DE without cleanup |
| 5 | local | Windows Editor | Compatible | DE-DE | either | character fidelity without editor noise |
| 6 | Horizon → vCenter → VM | Windows Editor | Compatible | Auto | either | the baseline defect (comparison run) |
| 7 | Horizon → vCenter → VM | Windows Editor | Compatible | DE-DE | either | the actual fix |

Run 2 is the control: if the indentation does *not* grow there, Notepad++ auto-indent is
off and runs 1, 3, 4 prove nothing. Run 6 is not optional — without it, run 7 cannot tell
a fix apart from a session that passes everything through anyway. Strip Auto-Indent is a
dead axis in the VDI because the Windows Editor never indents. Standard mode in the VDI
is deliberately absent: that is the Unicode path, and its failure there is the reason
Compatible mode exists.

The target VM's layout has to actually be German, or runs 6 and 7 say nothing.

Compatible mode additionally has **Target Layout** (Settings → Advanced), which selects
which layout the *target* console runs. It only matters against a real remote target, so
run it once per layout you actually use:

| Target Layout | What to expect on lines 05 and 07 |
|---|---|
| Auto | Resolved through your local layout via `VkKeyScanW`. On de-DE, `€ é à ñ` fall back to Unicode; measured in a nested Horizon session, `é à ñ` drop out there while `€` survives. |
| EN-US | `ö ä ü ß € é à ñ` are not on the EN-US layout and fall back to Unicode. Everything ASCII must be exact. |
| DE-DE | `€ ² ³ µ ö ä ü ß` must arrive as real keystrokes, and so must `é à` (composed dead key + vowel). Only `ñ` stays Unicode — German T1 has no dead tilde. |

Line 07 (`é à ñ`) is the whole point of DE-DE: under Auto these are the characters that
work on your desk and fail in the VDI session. Line 05 (`€`) is the documented
near-miss — same Unicode fallback, but it came through intact in the measured run. Also re-check lines 02–04
under DE-DE — the dead-key set there is hardcoded, not probed, so a wrong entry shows up
as a missing or doubled accent.

Use two targets. Windows Editor never auto-indents, so it can only prove character
fidelity. Notepad++ does auto-indent and is the only target that can prove the
auto-indent handling — but disable its word completion first, see below.

## What each line covers

| Line | Covers |
|---|---|
| 02–04 | **Dead keys.** `` ` ``, `´`, `^` must survive on their own. Line 03 is the combining test: `^a` must not become `â`. |
| 05 | **AltGr characters.** The vkey path sends a real right Alt (`VK_RMENU` + `KEYEVENTF_EXTENDEDKEY`), not Ctrl+left Alt. Before that change `}` `²` `³` were dropped even locally. `€` is not on `VkKeyScanW` for de-DE and still falls back to Unicode, but survived the measured nested session. |
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
