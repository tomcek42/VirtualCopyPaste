---
id: T01
parent: S02
milestone: M002
provides: []
requires: []
affects: []
key_files: ["src-tauri/Cargo.toml", "src-tauri/target/release/virtual-copy-paste.exe", "src-tauri/target/release/bundle/nsis/Virtual Copy Paste_2.0.0_x64-setup.exe"]
key_decisions: ["Used aggressive release profile: strip=true, lto=true, codegen-units=1, opt-level='s'", "LTO build takes ~5 minutes but produces excellent results"]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Release build completed successfully. Measured file sizes: .exe 4.3MB (under 10MB target), NSIS installer 1.4MB. Both artifacts exist on disk."
completed_at: 2026-03-29T20:36:09.175Z
blocker_discovered: false
---

# T01: Release build configured — 4.3MB .exe and 1.4MB installer (98% reduction from 206MB debug).

> Release build configured — 4.3MB .exe and 1.4MB installer (98% reduction from 206MB debug).

## What Happened
---
id: T01
parent: S02
milestone: M002
key_files:
  - src-tauri/Cargo.toml
  - src-tauri/target/release/virtual-copy-paste.exe
  - src-tauri/target/release/bundle/nsis/Virtual Copy Paste_2.0.0_x64-setup.exe
key_decisions:
  - Used aggressive release profile: strip=true, lto=true, codegen-units=1, opt-level='s'
  - LTO build takes ~5 minutes but produces excellent results
duration: ""
verification_result: passed
completed_at: 2026-03-29T20:36:09.177Z
blocker_discovered: false
---

# T01: Release build configured — 4.3MB .exe and 1.4MB installer (98% reduction from 206MB debug).

**Release build configured — 4.3MB .exe and 1.4MB installer (98% reduction from 206MB debug).**

## What Happened

Added [profile.release] to Cargo.toml with strip=true, lto=true, codegen-units=1, opt-level='s'. First build timed out at 10 minutes (LTO link phase). Second build completed in ~5 minutes. Release .exe is 4.3MB (down from 206MB debug — 98% reduction). NSIS installer is 1.4MB. Both well under the 10MB target.

## Verification

Release build completed successfully. Measured file sizes: .exe 4.3MB (under 10MB target), NSIS installer 1.4MB. Both artifacts exist on disk.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npx tauri build` | 0 | ✅ pass — release build + NSIS installer created | 291000ms |
| 2 | `ls -la src-tauri/target/release/virtual-copy-paste.exe` | 0 | ✅ pass — 4.3MB (under 10MB target) | 50ms |
| 3 | `ls -la 'src-tauri/target/release/bundle/nsis/Virtual Copy Paste_2.0.0_x64-setup.exe'` | 0 | ✅ pass — 1.4MB installer | 50ms |


## Deviations

None.

## Known Issues

Tauri warns that bundle identifier 'com.virtualcopypaste.app' ending in '.app' conflicts with macOS bundle extension — not relevant for Windows-only tool but worth fixing eventually.

## Files Created/Modified

- `src-tauri/Cargo.toml`
- `src-tauri/target/release/virtual-copy-paste.exe`
- `src-tauri/target/release/bundle/nsis/Virtual Copy Paste_2.0.0_x64-setup.exe`


## Deviations
None.

## Known Issues
Tauri warns that bundle identifier 'com.virtualcopypaste.app' ending in '.app' conflicts with macOS bundle extension — not relevant for Windows-only tool but worth fixing eventually.
