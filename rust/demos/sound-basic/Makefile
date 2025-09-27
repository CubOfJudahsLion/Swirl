PROFILE ?= dev

.PHONY: clean

html/sound_basic_bg.wasm html/sound_basic.js: pkg/sound_basic_bg.wasm pkg/sound_basic.js
	cp pkg/sound_basic{_bg.wasm,.js} html/

pkg/sound_basic_bg.wasm pgk/sound_basic.js: src/lib.rs
	reset
	wasm-pack build --$(PROFILE) --no-typescript -t web

clean:
	-rm {pkg,html}/sound_basic{_bg.wasm,.js}
