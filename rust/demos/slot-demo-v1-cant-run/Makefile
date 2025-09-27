.DUMMY: all

ifdef RELEASE
	BUILD_FLAGS := --release
else
	BUILD_FLAGS := --dev
endif

all:
	wasm-pack build ${BUILD_FLAGS} --no-typescript --out-dir out --target web
