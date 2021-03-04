all: build

build:
	cargo install wasm-pack
	wasm-pack build
	npm install --prefix www
	npm run build --prefix www
