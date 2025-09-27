# Author	: Alexander Feterman-Naranjo
# Created	: Wed Feb 17 11:51:19 PM CST 2021
# Modified	: Thu Feb 18 07:29:11 AM CST 2021


# Manually set PROF (profile) to REL for a release build or to nothing (or anything
# other than DEV) for a debug build.

# Basic variables
SRC_DIR 	:=	src-cpp
OUT_DIR 	:=	out
LAST_PROF_FILE	:=	LAST-PROF

# Set different compilation profiles (DEV = debuggable, REL = release.)
ifeq ($(PROF),REL)
	LINK_FLAGS	:= -O3 -g0 -flto
	COMPILE_FLAGS	:= -O3 -flto
else
	PROF		:= DEV
	LINK_FLAGS	:= -g4 --source-map-base http://192.168.50.116/GameEngines/engines/SL5R_WASM_MonkeyKhan_Test/
	COMPILE_FLAGS	:= -g -O0
	MAP_FILE	:= slots-wasm-test.wasm.map
	MAP_SRC		:= $(SRC_DIR)/$(MAP_FILE)
	MAP_DEST	:= $(MAP_FILE)
	MAP_COMMAND	:= cp $(MAP_SRC) $(MAP_DEST)
endif


# A canned command to erase target and temp files. We'll use this repeatedly, and it's
# a better idea to have a macro than running `emmake make` recursively.
CLEAN_CMD	:=	rm $(SRC_DIR)/*.{o,a,js,wasm,map} &> /dev/null; \
			rm $(OUT_DIR)/slots-wasm-test.{js,wasm} &> /dev/null; \
			rm *.map &> /dev/null;


# Check if we've changed compilation profiles (we store that in a file whose name is assigned
# to the variable LAST_PROF_FILE.) If there's no file, we create it. If the profile has changed,
# update source file dates so everything is recompiled.
#_DUMMY		:=	$(shell	if [[ ! (-f $(LAST_PROF_FILE)) ]]; then \
#					echo $(PROF) > $(LAST_PROF_FILE); \
#				elif [[ `cat $(LAST_PROF_FILE)` != $(PROF) ]]; then \
#					echo $(PROF) > $(LAST_PROF_FILE); \
#					$(CLEAN_CMD) \
#				fi)

.PHONY: clean

$(OUT_DIR)/slots-wasm-test.wasm $(OUT_DIR)/slots-wasm-test.js $(MAP_DEST) &: $(LAST_PROF_FILE) $(SRC_DIR)/slots-wasm-test.wasm $(SRC_DIR)/slots-wasm-test.js $(MAP_SRC)
	@cp $(filter-out %.map,$^) $(@D)
	@$(MAP_COMMAND)

$(SRC_DIR)/slots-wasm-test.wasm $(SRC_DIR)/slots-wasm-test.js: $(patsubst %.cpp,%.o,$(wildcard $(SRC_DIR)/*.cpp))
	@echo [Linking] $(notdir $@)
	@$(CXX)	$(LINK_FLAGS) --std=c++17 \
		-s MALLOC="emmalloc" -s INITIAL_MEMORY=1048576 -s MAXIMUM_MEMORY=16777216 \
		-s ENVIRONMENT=web -s DISABLE_EXCEPTION_CATCHING=0 -s FILESYSTEM=0 -s STRICT_JS=1 \
		-o $(SRC_DIR)/slots-wasm-test.js $^

$(SRC_DIR)/%.o: $(SRC_DIR)/%.cpp
	@echo [Compiling] $(notdir $@)
	@$(CXX)	$(COMPILE_FLAGS) --std=c++17 \
		-s DISABLE_EXCEPTION_CATCHING=0 -s FILESYSTEM=0 \
		-c -o $@ $<

clean:
	-@$(CLEAN_CMD)
