# Project Knowledge

Append-only register of project-specific rules, patterns, and lessons learned.
Agents read this before every unit. Add entries when you discover something worth remembering.

## Rules

| # | Scope | Rule | Why | Added |
|---|-------|------|-----|-------|

## Patterns

| # | Pattern | Where | Notes |
|---|---------|-------|-------|

## Lessons Learned

| # | What Happened | Root Cause | Fix | Scope |
|---|--------------|------------|-----|-------|
| MEM004 | macOS port code (M007) is in master but the user does NOT want macOS builds published. The release workflow builds both Windows + macOS. Any release tag on master will produce macOS artifacts. Either the workflow must be changed to Windows-only, or macOS code needs a separate branch. | — | — | project |
| MEM005 | macOS port code has been removed from master (commit 6856e97) and preserved on the 'macos-port' branch. The release workflow is now Windows-only. This resolves the earlier MEM004 gotcha about accidental macOS builds. | — | — | project |
