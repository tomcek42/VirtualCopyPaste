# S04: Multi-Slot Clipboard + Global Hotkeys — UAT

**Milestone:** M001
**Written:** 2026-03-28T23:57:41.812Z

## S04 UAT: Multi-Slot Clipboard + Global Hotkeys\n\n### Test 1: Slot Switching\n- [ ] Type 'Hello' in slot 1\n- [ ] Click slot 2 button, type 'World'\n- [ ] Click slot 1 — text shows 'Hello'\n- [ ] Click slot 2 — text shows 'World'\n- [ ] Slots with content show green dot indicator\n\n### Test 2: Slot Paste\n- [ ] Store text in slot 1, click Paste\n- [ ] Text from slot 1 appears in target window\n- [ ] Switch to slot 2, paste — different text appears\n\n### Test 3: Global Hotkeys\n- [ ] Open Notepad\n- [ ] Press Ctrl+Shift+1 — slot 1 content types into Notepad\n- [ ] Press Ctrl+Shift+2 — slot 2 content types into Notepad\n- [ ] Press Ctrl+Shift+3 — slot 3 content types into Notepad\n\n### Test 4: Hotkey Registration\n- [ ] Check app console output shows 'Registered global shortcut' for all 3
