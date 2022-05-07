# rust-stm32f051
Blinking example for STM32F0 Discovery Board written in Rust

## Dependencies
### Hardware
STM32F0-discovery using STM32F051R8 (Cortex M0)

### Software preparation for NixOS
Install Packages:
```Shell
environment.systemPackages = with pkgs; [ rustup stlink ];
```

Add udev rules for stlink
```Shell
services.udev.packages = [ pkgs.stlink];
```

Install thumb instructions for Cortex M0 core and prepare binutils
```Shell
rustup install stable
rustup default stable
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

## Building the target
```Shell
make
```

## Download firmware to the flash
1. Ensure USB cable is connected
2. Check if stlink was detected
```Shell
dmesg | tail
```
3. Ensure Jumpers CN2 are still set (is default), so the programmer is connected to the onboard chip.
4. Actually perform the download
```Shell
make download
```
