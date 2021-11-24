#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

// #[allow(unused)]
// use panic_halt;

#[allow(unused)]
use panic_semihosting;

use cortex_m_rt::entry;
use stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::lsi());
    let mut delay = dp.TIM16.delay(&mut rcc);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa12.into_push_pull_output();

    loop {
        // led.toggle().unwrap();
        led.set_high().unwrap();
        delay.delay(150.ms());
        led.set_low().unwrap();
        delay.delay(850.ms());
    }
}
