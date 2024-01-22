tools-install:
	rustup target add wasm32-unknown-unknown

build:
	cargo build --target wasm32-unknown-unknown
