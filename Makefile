all: build

build:
	cargo install wasm-pack
	wasm-pack build
	pushd client
	npm install
	npm run build
