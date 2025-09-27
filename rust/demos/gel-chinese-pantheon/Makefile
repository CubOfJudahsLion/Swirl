.PHONY: dev rel cleanfinal cleanpkg cleanall

dev : PROFILE = --dev
rel : PROFILE = --release

TEST_DIR := /f/WASM/GameTesters/PanteonChino/
TEST_FILES := $(addprefix $(TEST_DIR),wasm-game-loader.js game.wasm)
WASM_DIR := /f/WASM/Games/PanteonChino/
WASM_FILES := $(addprefix $(WASM_DIR),wasm-game-loader.js game.wasm)
PKG_DIR := ./pkg/
PKG_FILES := $(addprefix $(PKG_DIR),$(addprefix gel_chinese_pantheon,.js _bg.wasm))
SRC_DIR := ./src/
SRC_FILES := $(addprefix $(SRC_DIR),lib.rs assets.rs game.rs game_builder.rs reels.rs) $(wildcard ../../swirl/src/**/*.rs)

COMPILE = wasm-pack build -t no-modules $(PROFILE) --no-typescript

dev: $(TEST_FILES) $(WASM_FILES)

rel: $(TEST_FILES) $(WASM_FILES)

$(TEST_FILES) $(WASM_FILES): $(PKG_FILES)
	$(subst =, ,$(join $(addprefix cp=,$(PKG_FILES)),$(addsuffix ; ,$(addprefix =,$(TEST_FILES)))))
	$(subst =, ,$(join $(addprefix cp=,$(PKG_FILES)),$(addsuffix ; ,$(addprefix =,$(WASM_FILES)))))

$(PKG_FILES): $(SRC_FILES)
	$(COMPILE)

cleanfinal:
	@-rm $(TEST_FILES)
	@-rm $(WASM_FILES)

cleanpkg:
	@-rm $(PKG_FILES)

cleanall: cleanfinal cleanpkg
	@-rm -rf target
