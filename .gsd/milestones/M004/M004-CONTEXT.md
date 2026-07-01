# M004: Hotkey & Paste Reliability

**Gathered:** 2026-04-24
**Status:** Ready for planning

## Project Description

Virtual Copy Paste (v2.2.0) is a Windows/Tauri app that types clipboard text into VDI/Remote target windows via a global hotkey. Two keyboard modes exist: `unicode` (fast, uses `KEYEVENTF_UNICODE`) and `vkey` / Compatible (emulates real key presses via `VkKeyScanW` — required when target guests reject unicode injection).

## Why This Milestone

During live testing, three reliability bugs surfaced that together make the app feel unreliable in its primary use case (VDI paste):

1. **Hotkey vanishes after Windows suspend/wake** — user has to restart the app to get it back.
2. **Compatible mode mistypes** — missing first character, wrong case (shift state leaks), occasional dropped letters. Manifests under VDI where timing is tighter.
3. **Hotkey does not survive an app restart** — the saved custom hotkey is ignored; the hardcoded default registers instead.

Root cause for (3) already confirmed: `src-tauri/src/main.rs:528` uses `app.get_store("settings.json")` which only returns an already-loaded store. At setup time the store is not yet loaded, so `get_store` returns `None`, and the default hotkey is registered unconditionally. Reproduced in console log: `Registered activate hotkey: Ctrl+Shift+Space` after restart despite a custom hotkey being saved.

A fourth item is a UX request that becomes more important while (2) is unresolved: a one-click way to flip between `unicode` and `vkey` modes from the main window.

## User-Visible Outcome

### When this milestone is complete, the user can:

- Set a custom hotkey, quit the app, restart it, and the custom hotkey still works — no manual re-entry.
- Put Windows to sleep, resume, and trigger the hotkey immediately — no restart needed.
- Paste into a VDI/Remote target in Compatible mode and get the full text typed correctly, including the first character and correct case, without modifier-state bleed-through from the physical hotkey press.
- Flip between Unicode and Compatible keyboard modes from the main window in one click — no Settings round-trip.

### Entry point / environment

- Entry point: Tauri desktop app, global hotkey (default `Ctrl+Shift+Space`) + tray icon.
- Environment: Windows 10/11, often paired with VDI/RDP/Citrix guests as paste targets.
- Live dependencies involved: Win32 APIs (`SendInput`, `RegisterHotKey` via `tauri-plugin-global-shortcut`, `WM_POWERBROADCAST`), `tauri-plugin-store` for persistence.

## Completion Class

- **Contract complete means:** Rust code compiles cleanly; the store-load path in `main.rs` returns the saved hotkey when the on-disk `settings.json` contains one; shift-state clearing logic is unit-inspectable (no unit-test framework wired up, so: code-read + logs).
- **Integration complete means:** Real user flow works on the user's Windows machine for all four scenarios below — manual verification with the live app, not just compilation.
- **Operational complete means:** Hotkey survives both a clean app restart and a real Windows suspend/wake cycle. Health-check auto-recovers if the shortcut becomes unregistered between checks.

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- User sets a custom hotkey (e.g. `Ctrl+Alt+Shift+V`), saves, quits via tray, relaunches → console log shows `Registered activate hotkey: Ctrl+Alt+Shift+V` and the hotkey actually triggers the overlay.
- User holds the rechner sleeps (Fn+closes lid or sleep from Start menu), wakes it, and the saved hotkey triggers the overlay within ≤2s of first press without an app restart.
- User presses hotkey with physical Shift still held, text containing both uppercase and lowercase letters types correctly in Compatible mode into Notepad and a VDI target — first character present, case correct throughout.
- User clicks the in-app mode toggle, types a character that only `vkey` can produce (e.g. a layout-specific key), then switches back to Unicode and types an emoji — both work without opening Settings.

## Architectural Decisions

### D00X — Use `app.store()` instead of `app.get_store()` at startup

**Decision:** Load the store at setup time via `app.store("settings.json")` (which lazy-loads from disk) instead of `app.get_store(...)` (which only returns already-loaded stores).

**Rationale:** The plugin's own doc-comment warns that `get_store` returns `None` when the store is not in the in-memory registry — which is exactly the case at Tauri `setup()` time. The change is two lines and has no compatibility surface.

**Alternatives Considered:**
- Manually call `.load()` on a fresh `StoreBuilder` — more code, no upside over `app.store()`.
- Defer hotkey registration until the main webview reports ready — delays the hotkey availability for ~200-500ms after launch; worse UX.

### D00X — Combine resume-event hook with periodic health-check for shortcut resilience

**Decision:** Hook `WM_POWERBROADCAST` / `PBT_APMRESUMEAUTOMATIC` as the primary recovery trigger, plus a low-frequency (30s) health-check as safety net. On either trigger, unregister-and-reregister the active hotkey.

**Rationale:** `WM_POWERBROADCAST` is not universally delivered (depends on power scheme, modern standby vs legacy S3, driver quirks). Health-check alone would recover within 30s but feels laggy right after wake. Combined gives fast recovery in the common case and a backstop for the edge cases.

**Alternatives Considered:**
- Health-check only — simpler but up to 30s delay after wake.
- Resume-event only — misses modern-standby / connected-standby resume on some hardware.

### D00X — Compatible mode: flush physical modifier state before typing

**Decision:** Before sending the first character in `vkey` mode, explicitly send keyup for `VK_SHIFT`, `VK_CONTROL`, `VK_MENU` (both L/R) to clear any modifier the user is still physically holding from the hotkey press. Also add a small (5-10ms) pause between modifier-down and key-down inside `send_vkey_char`.

**Rationale:** The hotkey is typically `Ctrl+Shift+Space`. If the user releases slowly, `SendInput` synthetic presses race against real keystate. The flush is a one-time 6-key sequence at paste-start; cheap and safe.

**Alternatives Considered:**
- Poll `GetAsyncKeyState` and wait for user to release — unbounded wait, bad UX.
- Send no modifier events at all in Compatible mode — would break everything that needs Shift.

## Error Handling Strategy

- Store load failures log to stderr with the full error and fall back to default hotkey. The user sees the app start; the Settings window still works and can overwrite the default.
- Hotkey registration failures log to stderr with the OS error (already the case today). Health-check will retry on the next tick.
- Paste-time modifier flush is best-effort: if `SendInput` returns 0, log at debug level and continue — a failed flush is strictly better than no flush, and a noisy log would spam during every paste.

## Risks and Unknowns

- **WM_POWERBROADCAST delivery on Tauri's main thread** — Tauri owns the main window message pump. Need to confirm whether we can sub-class the HWND, or whether we attach via the existing Tauri window-event hook. Unknown until spiked in S02.
- **First-character-missing bug may have a second cause** — `wait_for_user_click` + 150ms sleep pattern means the first keydown arrives very soon after focus transfer. In some VDI clients the guest's input queue isn't primed yet. Modifier-flush alone may not fully fix (1); an additional configurable post-click delay is the lever.
- **Health-check overhead** — a 30s poll of `GlobalShortcut::is_registered` (if that exists in the plugin; to be confirmed) is negligible, but we need to confirm the API shape.
- **Tauri v2 plugin APIs change between minor versions** — the current `Cargo.toml` pins `tauri-plugin-store` to a known version; any re-register path must work against that exact version.

## Existing Codebase / Prior Art

- `src-tauri/src/main.rs:338-359` — `update_hotkey` command, the working unregister+register path. Reference implementation for health-check.
- `src-tauri/src/main.rs:363-408` — `type_text` command, where modifier flush and configurable post-click delay will be wired.
- `src-tauri/src/main.rs:205-293` — `send_vkey_char`, where per-modifier pauses go.
- `src-tauri/src/main.rs:523-549` — hotkey registration at setup, the S01 fix target.
- `src/settings.js:211-222` — frontend save flow; already correct, verified during this planning session.
- `src/app.js`, `src/index.html`, `src/styles.css` — main window UI, where the S04 mode toggle lands.

## Relevant Requirements

No formal requirements exist in `.gsd/REQUIREMENTS.md` yet. This milestone will surface at least:
- Hotkey persistence across restart and resume (operational).
- Faithful keypress reproduction in Compatible mode (functional, VDI target class).
- In-app mode toggle (functional, ergonomics).

These will be captured as R### entries during slice execution where they become concrete.

## Scope

### In Scope

- Fix for the load-time `get_store` bug so the persisted hotkey registers on launch.
- Resume-event hook + periodic health-check with auto re-register.
- Pre-paste modifier flush, per-modifier timing pause inside `send_vkey_char`, configurable post-click delay.
- Minimalist segmented toggle in the main window for Unicode/Compatible mode, reflecting and writing the store value live.

### Out of Scope / Non-Goals

- macOS / Linux support.
- New keyboard layouts or language profiles beyond what `VkKeyScanW` already supports.
- Clipboard history, profiles per target, or other structural features.
- Rewriting the paste pipeline to use the clipboard + `Ctrl+V` (explicitly avoided — the app exists because that does not work in VDI).
- Introducing a unit/integration test harness (can be a follow-up milestone).

## Technical Constraints

- Tauri v2; plugins are `tauri-plugin-store`, `tauri-plugin-autostart`, `tauri-plugin-global-shortcut`.
- Windows-only code paths are gated with `#[cfg(windows)]`; other targets must still compile (keep the `cfg(not(windows))` arm of `type_text`).
- No new heavy dependencies — prefer stdlib + existing `windows` crate facets already in use.
- UI changes in the main window must match the existing visual language (dark, minimalist, the Yeti animation area is sacred — don't crowd it).

## Integration Points

- `tauri-plugin-global-shortcut` — register / unregister / (if available) is_registered query.
- `tauri-plugin-store` — load at startup, read hotkey and keyboard-mode values.
- Win32 `SendInput`, `RegisterPowerSettingNotification` / window message `WM_POWERBROADCAST` — resume hook.
- Main-window frontend (`app.js` / `index.html` / `styles.css`) — mode toggle UI.

## Testing Requirements

No automated test framework is wired up. Verification is manual, scenario-based, per slice:

- **S01:** Set custom hotkey, quit, relaunch, observe console log `Registered activate hotkey: <custom>`; press hotkey, overlay appears.
- **S02:** Force a suspend (or unregister the hotkey out-of-band to simulate), verify health-check re-registers; test real sleep/wake cycle.
- **S03:** Press hotkey while physically holding Shift, paste mixed-case text into Notepad and a VDI target (if available); confirm first character present and case correct. Test with post-click delay at 150/250/400ms.
- **S04:** Click toggle, paste, click toggle, paste again; confirm store is updated (check `settings.json` on disk) and behavior actually changes between pastes.

## Acceptance Criteria

Per slice (see ROADMAP). The milestone is accepted when all four "Final Integrated Acceptance" scenarios above pass on the user's machine.

## Open Questions

- Does `tauri-plugin-global-shortcut` expose an `is_registered(accelerator)` query, or do we probe by attempting to register and observing the error? — to be resolved in S02.
- What is the right default for the post-click delay? 250ms is a reasonable starting point; may need to be layout/target-dependent. Default + user-configurable override in Settings.
- Does the resume-event handler need to run on the Tauri main thread, or can it live on its own message-pump thread? — to be spiked in S02.
