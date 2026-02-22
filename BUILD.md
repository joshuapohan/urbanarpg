# Build Steps

## Prerequisites

- [Rust](https://rustup.rs/) with the nightly toolchain (see `dodge/rust-toolchain.toml`)
- [Emscripten (emsdk)](https://emscripten.org/docs/getting_started/downloads.html) — required for WASM builds
- [Godot 4](https://godotengine.org/) — for running the project

## Rust / Native (Windows)

```bash
cd dodge
cargo build
```

Output: `dodge/target/debug/dodge.dll`

## WASM (Web export)

Emscripten must be activated in the current shell before building:

```bash
# Activate emsdk (adjust path as needed)
source /path/to/emsdk/emsdk_env.sh

cd dodge
cargo build --target wasm32-unknown-emscripten
```

Output: `dodge/target/wasm32-unknown-emscripten/debug/dodge.wasm`

The `.gdextension` file already points to the correct relative paths for both targets.

## Godot

Open `godot/dodge-the-creeps-rust/` in Godot 4. The extension is loaded automatically via `dodgerust.gdextension`.

For a web export, use Godot's **Export** dialog with the Web preset. The exported files land in `godot/dodge-the-creeps-rust/` alongside the `.wasm` file built above.
