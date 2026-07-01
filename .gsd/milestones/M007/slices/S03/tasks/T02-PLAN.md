---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T02: Implemented wait_for_user_click_macos() with atomic counter and listen-only CGEventTap

Create a macOS-specific AtomicU32 CLICK_COUNT_MACOS static. Implement an extern C callback that increments the counter on kCGEventLeftMouseDown. Implement wait_for_user_click_macos(required_clicks: u32, timeout: Duration) -> Result<(), String> that: resets counter, creates a listen-only CGEventTap, creates a run loop source, adds it to the current run loop, polls in short intervals checking counter vs required_clicks and elapsed vs timeout, then cleans up (remove source, invalidate port, release).

## Inputs

- `src-tauri/src/main.rs (cg module with FFI bindings from T01)`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check passes
