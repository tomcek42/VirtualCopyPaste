# S01 Assessment

**Milestone:** M004
**Slice:** S01
**Completed Slice:** S01
**Verdict:** roadmap-adjusted
**Created:** 2026-04-30T15:46:07.394Z

## Assessment

User decided to skip ahead from S01 to S04 (in-app keyboard-mode toggle) before tackling S02 (suspend/wake) and S03 (Compatible-mode reliability). Rationale: S04 is small UI work that only needs the working store-load path delivered by S01 — it does not technically require the Compatible-mode fixes from S03. The toggle's value is independent of whether Compatible mode types reliably; users may want the toggle even with current Compatible-mode behavior, and exposing it sooner gives them control to switch modes per-target without a Settings round-trip. S02 and S03 remain in the roadmap, just deferred. Drop S04→S03 dependency; keep S04→S01 (store-load fix is genuinely required because the toggle reads/writes keyboardMode through the same store path). The S03→S04 boundary contract that "S03 confirms keyboardMode store key shape" is unnecessary — the key shape is already established by Settings (string 'unicode' | 'compatible').
