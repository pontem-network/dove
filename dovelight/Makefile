build_web:
	cargo build --lib --target wasm32-unknown-unknown --release --features "internal_rt"
	wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir target/wasm32-unknown-unknown/release/wasm/web --target web
build_js:
	cargo build --lib --target wasm32-unknown-unknown --release --features "external_rt"
	wasm-bindgen target/wasm32-unknown-unknown/release/dovelight.wasm --out-dir target/wasm32-unknown-unknown/release/wasm/js --target nodejs
wasi_external_rt:
	cargo build --lib --target wasm32-wasi --release --features "external_rt"
wasi_internal_rt:
	cargo build --lib --target wasm32-wasi --release --features "internal_rt"