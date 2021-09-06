cargo build --lib --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/client.wasm --out-dir ./static/pkg --target web
