.PHONY: RUST_SOURCE

pkg: RUST_SOURCE
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen --target web \
	             --out-dir ./pkg \
	             ./target/wasm32-unknown-unknown/release/lazaretto.wasm
