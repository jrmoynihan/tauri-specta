# AGENTS.md

`tauri-specta` is a Rust library that generates typesafe TypeScript/JavaScript bindings for Tauri commands and events. The repo is a Cargo workspace (root crate + `macros/`) plus runnable Tauri demos under `examples/`, wired together with a `pnpm` workspace for the frontend halves.

See `README.md` for the canonical run/test commands.

## Cursor Cloud specific instructions

### Environment facts
- The crate uses Rust `edition = "2024"`, which requires a recent stable toolchain (>= 1.85). The startup/update script installs and defaults to `stable` via `rustup`; do not pin the old base-image toolchain.
- Tauri's Linux backend needs system libraries (`libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `libsoup-3.0-dev`). These are provided by the VM snapshot, not the update script. If a build fails with missing `gtk`/`webkit2gtk`/`soup` pkg-config errors, reinstall them with `apt-get`.

### Build / lint / test
- Build everything: `cargo build --all-features`.
- Lint: `cargo clippy --all-features` (workspace lints treat `unwrap_used`, `panic`, `todo`, etc. as warnings).
- Tests: `cargo test` requires `OUT_DIR` to be set (used to write generated bindings during tests):
  ```bash
  mkdir -p _out
  OUT_DIR="$(pwd)/_out" cargo test --all --all-features
  ```
- Gotcha: two library doctests (`src/lib.rs` JSDoc example and `src/builder.rs` `export` example) call `.export(..., "../src/...")`. The path is relative to the crate root, so it resolves to `/src` when the repo is checked out at `/workspace`, where `/` is read-only — those two doctests fail with `Permission denied (os error 13)`. This is an environment artifact, not a code bug; lib + integration tests (`cargo test --lib --tests`) pass cleanly. In normal CI the repo lives in a deeper, writable path so the parent dir exists and the doctests pass.

### Running the example app (GUI)
- A virtual X display is available at `DISPLAY=:1` (used by computer-use). Launch the demo with that display set:
  ```bash
  cd examples/app
  DISPLAY=:1 pnpm tauri dev
  ```
- `pnpm tauri dev` runs `pnpm dev` (Vite on port 1420) as `beforeDevCommand`, then `cargo run` for `examples/app/src-tauri`, then opens a WebKitGTK window.
- `libEGL ... DRI3` warnings on startup are expected (software rendering fallback) and do not prevent the window from rendering.
- The app's "Greet" button invokes the Rust `hello_world` command through the generated bindings and shows `Hello, <name>! You've been greeted from Rust!` — a good end-to-end smoke check.
- There is a second demo at `examples/custom-plugin/app`.
