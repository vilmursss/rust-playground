TARGET = thumbv8m.main-none-eabi
FLASH_TOOL = st-flash

RUSTC = rustc
CARGO = cargo

OBJCOPY = arm-none-eabi-objcopy

# Output binary name
OUT = target/$(TARGET)/release/stm32_rgb_blink

all: build

build:
	$(CARGO) build --release --target $(TARGET)

# Convert the ELF file to a binary file
$(OUT).bin: $(OUT)
	$(OBJCOPY) -O binary $< $@

# Flash the binary to the board
flash: build $(OUT).bin
	st-flash --reset write $(OUT).bin 0x08000000

clean:
	$(CARGO) clean

.PHONY: all build flash clean
