# `Blinky on the stm32g031j6`

A simple rust blinky example on a stm32g031j6 microcontroller on the
[stm32g0316-disco](https://www.st.com/en/evaluation-tools/stm32g0316-disco.html)
development board. The board includes a st-link v2 programmer.

![stm32g0316-disco](https://www.st.com/bin/ecommerce/api/image.PF267507.en.feature-description-include-personalized-no-cpn-medium.jpg)

## Quick Start

### Get dependencies:

- get stlink (currently 1.5.1 is not enough, a patched one
  is required: https://github.com/texane/stlink/pull/857)

- install cortex-m0+ support for rust: `rustup target add thumbv6m-none-eabi`

- `arm-none-eabi-binutils` is mandatory, `arm-none-eabi-gdb` is helpful

### Compile and flash:

```
cargo build

arm-none-eabi-objcopy -Obinary \
    target/thumbv6m-none-eabi/debug/stm32g0-disco-rs     \
    target/thumbv6m-none-eabi/debug/stm32g0-disco-rs.bin

path-to-stlink/build/Release/st-flash --debug write \
    target/thumbv6m-none-eabi/debug/stm32g0-disco-rs.bin 0x8000000
```

Alternatively, `cargo-flash` should get stm32g0 support soon, and it supports
flashing ELF files directly, so you wont need neither `arm-none-eabi-objcopy`
(`arm-none-eabi-binutils`) nor `stlink`.


## Notes:

* `memory.x` - is the memory layout of the stm32g031j6 mcu (8KB ram, 32KB flash).
  That layout assumes no bootloader, with a bootloader the FLASH ORIGIN needs
  to be moved to something like 0x08002000 for a 8KB bootloader. Not that I
  know of a bootloader for the stm32g031j6.
* `.cargo/config` - is setup to automatically select the thumbv6m-none-eabi (cortex-m0+)
  target on build.


## See also:

* [libopencm3 miniblink](https://github.com/libopencm3/libopencm3-miniblink) - blinkies for _any_ board supported. Useful for testing your toolchain and build environment
* [awesome embedded rust](https://github.com/rust-embedded/awesome-embedded-rust) - Curated list of resources for Embedded and Low-level development in the Rust programming language

