# S04: Accessibility permission check and user guidance — UAT

**Milestone:** M007
**Written:** 2026-06-24T21:36:09.897Z

| # | Check | Expected | Actual | Verdict |
|---|-------|----------|--------|---------|
| 1 | `cargo check` compiles without errors | exit 0 | exit 0 (3.35s) | PASS |
| 2 | `AXIsProcessTrusted` FFI binding exists in `cg` module | Binding declared | Present at line ~80 | PASS |
| 3 | `check_accessibility_permission` returns true on non-macOS | Always true | `#[cfg(not(target_os = "macos"))] { true }` | PASS |
| 4 | `open_accessibility_settings` uses correct URL scheme | `x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility` | Matches | PASS |
| 5 | Startup emits `accessibility-missing` if not trusted | Event emitted | Code at setup block emits event | PASS |
| 6 | Banner HTML/CSS present in index.html/styles.css | Elements exist | `#accessibilityNotice` div + `.accessibility-notice` styles | PASS |
| 7 | Focus listener re-checks permission | `tauri://focus` listener calls `check_accessibility_permission` | Present in app.js | PASS |
| 8 | Window height accounts for banner | `NOTICE_HEIGHT` added | 28px added when visible | PASS |

**Note:** Runtime macOS behavior cannot be verified on this Windows machine. Compile-time correctness confirmed via `cargo check` and code review.
