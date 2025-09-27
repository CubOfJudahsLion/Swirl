DEVREL ?= --dev

html/slot_demo.js html/slot_demo_bg.wasm: pkg/slot_demo.js pkg/slot_demo_bg.wasm
	cp pkg/slot_demo.js pkg/slot_demo_bg.wasm html

pkg/slot_demo.js pkg/slot_demo_bg.wasm: src/*.rs ../../swirl/src/**/*.rs
	wasm-pack build $(DEVREL) --target web --no-typescript

.PHONY: clean
clean:
	rm -rf pkg
	rm html/slot_demo{.js,_bg.wasm}
