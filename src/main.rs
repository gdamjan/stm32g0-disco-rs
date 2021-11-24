#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::lsi());
    let mut delay = dp.TIM16.delay(&mut rcc);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa12.into_push_pull_output();

    rprintln!("Hello, world!");
    loop {
        // led.toggle().unwrap();
        led.set_high().unwrap();
        delay.delay(150.ms());
        led.set_low().unwrap();
        delay.delay(850.ms());
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
