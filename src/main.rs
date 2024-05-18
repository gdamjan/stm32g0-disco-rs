#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;

use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::lsi());
    let mut delay = dp.TIM16.delay(&mut rcc);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.toggle().unwrap();
        delay.delay(500.millis());
    }
}
