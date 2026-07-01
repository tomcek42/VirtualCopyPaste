# S03: Mouse click detection via CGEventTap — UAT

**Milestone:** M007
**Written:** 2026-06-24T11:34:40.660Z

## UAT: S03 — Mouse click detection via CGEventTap

### Check 1: CGEventTap FFI bindings compile
- **Method:** cargo check on Windows (cross-compile gate)
- **Evidence:** cargo check exit 0, 4.90s
- **Verdict:** PASS

### Check 2: wait_for_user_click_macos function exists with correct signature
- **Method:** Code inspection — function takes (u32, Duration) -> Result<(), String>
- **Evidence:** Function at ~line 800 in main.rs, uses atomic counter + CGEventTap
- **Verdict:** PASS

### Check 3: Placeholder sleep replaced in type_text macOS path
- **Method:** Code inspection — no thread::sleep placeholder remains in Step 3
- **Evidence:** Line now calls wait_for_user_click_macos(required_clicks, Duration::from_secs(30))
- **Verdict:** PASS

### Check 4: Listen-only tap preserves click pass-through
- **Method:** Code inspection — KCG_EVENT_TAP_OPTION_LISTEN_ONLY = 1 used in CGEventTapCreate
- **Evidence:** Tap created with listen-only option, callback returns event unchanged
- **Verdict:** PASS

### Check 5: Double-click mode requires 2 clicks
- **Method:** Code inspection — required_clicks = if dbl_click { 2 } else { 1 }
- **Evidence:** Matches Windows pattern exactly
- **Verdict:** PASS

### Check 6: No Windows regressions
- **Method:** cargo check passes, Windows code unchanged
- **Evidence:** cargo check exit 0
- **Verdict:** PASS
