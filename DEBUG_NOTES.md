# Markdown Viewer Debug Notes

## Project Structure
- Tauri 2.x + Leptos 0.7 (WASM frontend) + Trunk build tool
- Frontend in `/src/`, Tauri backend in `/src-tauri/`

## Current Status
- **Frontend builds successfully** with `trunk build` (just deprecation warnings)
- **Backend compiles successfully** with `cargo check`
- Both compile without errors

## Known Issues to Fix

### 1. Leptos Deprecation Warnings (src/lib.rs)
Replace deprecated functions:
- `create_signal()` → `signal()`
- `create_effect()` → `Effect::new()`

### 2. File Dialog Bug (src-tauri/src/commands.rs)
The `select_markdown_file` function uses a callback-based approach with `thread::sleep(500ms)` which is unreliable. The dialog returns before the callback fires.

**Fix:** Use the blocking/sync version of the file dialog or proper async handling.

### 3. Running the App
To run: `cargo tauri dev` from the project root `/home/ekugic/markdown-viewer-but-better`

## Files to Modify for Fixes

1. **src/lib.rs** - Update deprecated Leptos API calls
2. **src-tauri/src/commands.rs** - Fix file dialog to work properly
3. **index.html** - CSS can be improved for modern look

## Build Commands
```bash
# Build frontend only
trunk build

# Run full app in dev mode
cargo tauri dev
```

## Dependencies Installed
- trunk 0.21.14
- wasm32-unknown-unknown target
- All Tauri dependencies compile successfully
