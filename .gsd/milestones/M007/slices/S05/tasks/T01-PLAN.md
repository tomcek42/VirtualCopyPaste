---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Added macOS build job to GitHub Actions release workflow for dual-platform artifacts

Add macOS build job to release.yml — runs-on macos-latest, builds universal binary (aarch64 + x86_64), produces .dmg alongside existing Windows NSIS installer

## Inputs

- `Existing release.yml (Windows-only)`

## Expected Output

- `.github/workflows/release.yml`

## Verification

cargo check passes, workflow YAML is valid
