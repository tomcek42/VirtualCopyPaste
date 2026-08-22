# Manual paste regression test

`paste-test.txt` is a fixture for the keyboard simulation path. Open it in an editor,
copy everything, paste it into Virtual Copy Paste (Multi-Line mode), then paste into the
target window and diff the result against the file.

21 lines, 648 characters — roughly 15 seconds at the default 20 ms typing delay.

Run it four times: **Standard** and **Compatible** keyboard mode, each with
*Strip Auto-Indent After Line Break* on and off.

## What each line covers

| Line | Covers |
|---|---|
| 02–04 | **Dead keys.** `` ` ``, `´`, `^` must survive on their own. Line 03 is the combining test: `^a` must not become `â`. |
| 05 | **AltGr characters.** These pass locally; in a remote session they are still expected to fail — the vkey path sends `VK_LMENU` instead of `VK_RMENU`. |
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
