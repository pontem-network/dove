build_js:
	cargo build --lib --target wasm32-unknown-unknown --release --no-default-features --features "js"
	wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir target/wasm32-unknown-unknown/release/wasm/pkg
wasi:
	cargo build --lib --target wasm32-wasi --release
