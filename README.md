# `Blinky on the stm32g031j6`

A simple rust blinky example on a stm32g031j6 microcontroller on the
[stm32g0316-disco](https://www.st.com/en/evaluation-tools/stm32g0316-disco.html)
development board. The board includes a st-link v2 programmer. The stm32g031j6 is
a quite small 8pin mcu, which has only 8KB ram and 32KB flash.

![stm32g0316-disco](https://github.com/gdamjan/stm32g0-disco-rs/assets/81654/01caaff3-19c2-4af8-b28a-5d0efaecba6e)

## Quick Start

### Setup tooling:

- [`rustup`](https://rustup.rs/) - is recommended to install rust and its components
- `cargo install cargo-binutils` - for `cargo size` and `cargo objdump -- --disassemble`, etc…
- `cargo install probe-rs-tools` - flash and debug using the [`probe-rs project`](https://probe.rs/)

### Compile and flash:

```
cargo embed --release
```

## Notes:

### `memory.x`

This file specifies the memory layout of the `stm32g031j6` mcu: 8KB sram starting at address 0x20000000
and 32KB of flash starting at address 0x08000000.

> That layout assumes no bootloader, with a bootloader the FLASH ORIGIN needs
to be moved to something like 0x08002000 for a 8KB bootloader. Not that I
know of a bootloader for the stm32g031j6.

### `.cargo/config`

This file holds the cargo build configuration. It is setup to build using the thumbv6m-none-eabi (cortex-m0)
target by default. Also, it'll show the program size (using `arm-none-eabi-size`) when
you run `cargo run`.

### `Embed.toml`

`Embed.toml` is configuration for the [cargo embed](https://probe.rs/docs/tools/cargo-embed/) tool.

### probe-run

See [probe-run](https://github.com/knurling-rs/probe-run) "Runs embedded programs just like native ones", just `cargo install probe-run` away.

[![asciicast](https://asciinema.org/a/452040.svg)](https://asciinema.org/a/452040)


### Tracing with gdb

Change the `runner` in `.cargo/config` to: `runner = 'arm-none-eabi-gdb'`, and
then in one terminal run:
```
cargo flash --chip stm32g031j6mx --gdb
```
and in another:
```
cargo run
```


### Using [stlink](https://github.com/stlink-org/stlink/)

`stlink` (1.6.1 and up) can also be used to flash the mcu on this board. stlink can't flash
ELF files directly, so you need a binary dump of the program code, and you have to
specify the address of the flash (`0x8000000`):

```
arm-none-eabi-objcopy -Obinary \
    target/thumbv6m-none-eabi/debug/stm32g0-disco-rs     \
    target/thumbv6m-none-eabi/debug/stm32g0-disco-rs.bin

st-flash --debug write \
    target/thumbv6m-none-eabi/debug/stm32g0-disco-rs.bin 0x8000000
```


## See also:

* [Discover the world of microcontrollers through Rust!](https://docs.rust-embedded.org/discovery/) - introductory book about rust on microcontrollers
* [awesome embedded rust](https://github.com/rust-embedded/awesome-embedded-rust) - Curated list of resources for Embedded and Low-level development in the Rust programming language
* [libopencm3 miniblink](https://github.com/libopencm3/libopencm3-miniblink) - blinkies for _any_ board supported. Useful for testing your toolchain and build environment
