ELF = rust-stm32f051

all: target

target: src/main.rs
	cargo build

clean:
	cargo clean
download: all
	# Generate binary
	cargo size --bin $(ELF) -- -A
	cargo objcopy --bin $(ELF) -- -Obinary /tmp/$(ELF).bin
	# Upload to device
	st-flash write /tmp/$(ELF).bin 0x8000000

