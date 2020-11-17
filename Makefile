ELF = rust-stm32f051

all: target

target: src/main.rs
	cargo build --release

clean:
	cargo clean
download: all
	# Generate binary
	cargo size --release --bin $(ELF) -- -A
	cargo objcopy --release --bin $(ELF) -- -Obinary /tmp/$(ELF).bin
	# Upload to device
	st-flash write /tmp/$(ELF).bin 0x8000000

