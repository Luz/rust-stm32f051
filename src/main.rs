#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt as _;

use stm32f0xx_hal as hal;

use hal::{pac, prelude::*, time::Hertz, timers::*};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    // Configure clock to 48 MHz and freeze it
    let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

    // Configure PC8 as output, named led
    let gpioc = p.GPIOC.split(&mut rcc);
    let mut led = cortex_m::interrupt::free(|cs| gpioc.pc8.into_push_pull_output(cs));

    let mut timer = Timer::tim1(p.TIM1, Hertz(10), &mut rcc);

    loop {
        led.set_high().ok();
        nb::block!(timer.wait()).ok();
        led.set_low().ok();
        nb::block!(timer.wait()).ok();
    }
}
