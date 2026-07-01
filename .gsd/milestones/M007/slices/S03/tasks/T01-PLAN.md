---
estimated_steps: 1
estimated_files: 1
skills_used: []
---

# T01: Added CGEventTap and CFRunLoop FFI bindings to the cg module for mouse click detection

Add the raw FFI declarations needed for CGEventTap click detection: CGEventTapCreate, CGEventTapEnable, CFMachPortCreateRunLoopSource, CFRunLoopGetCurrent, CFRunLoopAddSource, CFRunLoopRunInMode, CFRunLoopRemoveSource, CFMachPortInvalidate, CFRelease. Add constants for kCGEventLeftMouseDown mask, kCGHeadInsertEventTap, kCGEventTapOptionListenOnly, and kCFRunLoopDefaultMode. Add types for CFMachPortRef, CFRunLoopSourceRef, CFRunLoopRef, CGEventTapCallBack.

## Inputs

- `src-tauri/src/main.rs (cg module)`

## Expected Output

- `src-tauri/src/main.rs`

## Verification

cargo check passes
