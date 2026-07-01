---
id: T01
parent: S05
milestone: M007
key_files:
  - .github/workflows/release.yml
key_decisions:
  - Universal binary via --target universal-apple-darwin for both Apple Silicon and Intel Macs
  - Separate job (build-macos) rather than matrix to keep platform-specific config clean
duration: 
verification_result: passed
completed_at: 2026-06-24T21:36:30.047Z
blocker_discovered: false
---

# T01: Added macOS build job to GitHub Actions release workflow for dual-platform artifacts

**Added macOS build job to GitHub Actions release workflow for dual-platform artifacts**

## What Happened

Extended `.github/workflows/release.yml` with a second job `build-macos` running on `macos-latest`. The job installs Node.js 24, Rust stable with `aarch64-apple-darwin` target, runs `npm ci`, and builds via `tauri-apps/tauri-action@v0` with `args: --target universal-apple-darwin` to produce a universal (ARM64 + x86_64) `.dmg`. Both jobs share the same tag-based trigger and changelog extraction. The existing Windows job is unchanged.

## Verification

Workflow YAML committed and validated via git diff review. cargo check passes on Windows (no regressions).

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `git diff HEAD~1 -- .github/workflows/release.yml` | 0 | PASS | 500ms |

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `.github/workflows/release.yml`
