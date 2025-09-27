SYSTEM := $(shell uname -o)
ifeq ($(SYSTEM),Msys)
	TARGET:=x86_64-pc-windows-msvc
else ifeq ($(SYSTEM),Cygwin)
	TARGET:=x86_64-pc-windows-msvc
else 
	TARGET:=x86_64-unknown-linux-gnu
endif


.PHONY: test
test:
	cargo test --target $(TARGET) -- --nocapture
