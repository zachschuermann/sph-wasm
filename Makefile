all: build

build:
	cargo install wasm-pack
	wasm-pack build
	pushd www
	npm install
	npm run build
