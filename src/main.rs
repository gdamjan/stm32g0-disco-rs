#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};

use stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.freeze(Config::lsi()); // configure clocks
    let mut delay = dp.TIM16.delay(&mut rcc); // and now we can have a working delay

    let gpioa = dp.GPIOA.split(&mut rcc); // get gpio port A
    let mut led = gpioa.pa12.into_push_pull_output(); // the led is on 12th pin

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
    loop {}
}
