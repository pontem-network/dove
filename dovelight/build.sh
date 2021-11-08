cargo build --lib --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir ./static/pkg --target web

#cargo install cargo-wasi
cargo build --lib --target wasm32-wasi --release
