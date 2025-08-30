#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_itm as _;
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, gpio::GpioExt, delay::Delay, pac};
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::{ToggleableOutputSwitch};

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let mut led_array = [leds.ld3, leds.ld5, leds.ld7, leds.ld9, leds.ld10, leds.ld8, leds.ld6, leds.ld4];
    let compass_delay = 50u16;
    
    loop {
        for led in led_array.iter_mut() {
            led.toggle().ok();
            delay.delay_ms(compass_delay);
            led.toggle().ok();
            delay.delay_ms(compass_delay);
        }
    }
}

// use stm32f3_discovery::stm32f3xx_hal::{prelude::*, stm32, gpio::GpioExt};
// use stm32f3_discovery::stm32f3xx_hal::prelude::*;
// use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};