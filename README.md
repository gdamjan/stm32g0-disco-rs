# `Blinky on the stm32g031j6`

A simple rust blinky example on a stm32g031j6 microcontroller on the
[stm32g0316-disco](https://www.st.com/en/evaluation-tools/stm32g0316-disco.html)
development board. The board includes a st-link v2 programmer.

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


