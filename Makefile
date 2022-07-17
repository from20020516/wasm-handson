.PHONY: init build serve

init:
	npm --prefix hello-wasm/site install

build:
	wasm-pack build hello-wasm --scope from20020516

serve:
	npm --prefix hello-wasm/site run serve
