# With no task specified, we default to dev
DEVREL = --dev

html/sprite_movie.js html/sprite_movie_bg.wasm: pkg/sprite_movie.js pkg/sprite_movie_bg.wasm
	cp pkg/sprite_movie{.js,_bg.wasm} html

pkg/sprite_movie.js pkg_sprite_movie_bg.wasm: src/lib.rs
	wasm-pack build --no-typescript $(DEVREL) -t web

.PHONY: touch clean dev rel

touch:
	touch src/lib.rs

clean:
	-rm -rf pkg
	-rm html/sprite_movie{.js,_bg.wasm}

dev:
	$(MAKE) DEVREL=--dev

rel:
	$(MAKE) DEVREL=--release
