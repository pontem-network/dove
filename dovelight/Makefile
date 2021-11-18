build_web:
	cargo build --lib --target wasm32-unknown-unknown --release --features "web"
	wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir target/wasm32-unknown-unknown/release/wasm/web --target web
build_js:
	cargo build --lib --target wasm32-unknown-unknown --release --no-default-features --features "js"
	wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir target/wasm32-unknown-unknown/release/wasm/js
wasi:
	cargo build --lib --target wasm32-wasi --release