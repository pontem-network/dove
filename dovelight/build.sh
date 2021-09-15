cargo build --lib --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir ./pkg --target web
