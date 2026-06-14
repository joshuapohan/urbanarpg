build desktop
cargo build

build wasm
cargo build -Zbuild-std --target wasm32-unknown-emscripten 