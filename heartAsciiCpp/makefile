CPPC = clang++
MAIN_FILE = main.cpp
OUTPUT_BIN = binary/heart

build:
	mkdir -p $(dir $(OUTPUT_BIN))
	$(CPPC) $(MAIN_FILE) -o $(OUTPUT_BIN)

$(OUTPUT_BIN):
	$(MAKE) build

run: $(OUTPUT_BIN)
	echo -e "\n\n\n\n\n\n\n\n\n\n"
	$(OUTPUT_BIN)

.PHONY: build run