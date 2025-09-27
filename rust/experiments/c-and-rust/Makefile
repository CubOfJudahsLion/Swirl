SRC_DIR := src/
PKG_DIR := pkg/
FINAL_DIR := html/

COMPILER_OUTPUT := c_and_rust.js c_and_rust_bg.wasm
FINAL_FILES := $(addprefix $(FINAL_DIR),$(COMPILER_OUTPUT))
PKG_FILES := $(addprefix $(PKG_DIR),$(COMPILER_OUTPUT))

$(FINAL_FILES): $(PKG_FILES)
	cp $(PKG_FILES) $(FINAL_DIR)

$(PKG_FILES): $(addprefix $(SRC_DIR),lib.rs tinylib.c tinylib.h)
	wasm-pack build --dev -t web

.PHONY: clean reset

clean:
	-rm -rf pkg $(FINAL_FILES)

reset: clean
	-rm -rf target
