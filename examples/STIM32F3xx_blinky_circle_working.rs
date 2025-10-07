#![no_std]
#![no_main]

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, gpio::GpioExt, delay::Delay, pac};
use stm32f3_discovery::switch_hal::{ToggleableOutputSwitch};

extern crate panic_itm;

use cortex_m_rt::entry;


#[entry]
fn main() -> !{
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut rcc = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    
    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut rcc.ahb);
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
// 
    let mut led_array = 
    [leds.ld3, leds.ld5, leds.ld7, leds.ld9, leds.ld10, leds.ld8, leds.ld6, leds.ld4];
    let compass_delay = 50u16;
    // 
    loop {
        for led in led_array.iter_mut() {
            led.toggle().ok();
            delay.delay_ms(compass_delay);
            led.toggle().ok();
            delay.delay_ms(compass_delay);
        }
    }
}