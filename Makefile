all: build

build:
	cargo install wasm-pack
	wasm-pack build
	cd www
	npm install
	npm run build
