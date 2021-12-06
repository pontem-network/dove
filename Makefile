build-cli:
	cargo build
build-wasm-web:

build-wasm-js:

fmt:
	cargo fmt

test:
	cargo test --all

clippy:
	cargo clippy --tests --examples -- -Dwarnings

pre-commit: fmt clippy test build-cli build-wasm-js build-wasm-web


