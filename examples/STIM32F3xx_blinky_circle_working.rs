#![no_std]
#![no_main]

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, gpio::GpioExt, delay::Delay, pac};
use stm32f3_discovery::switch_hal::{ToggleableOutputSwitch};

extern crate panic_itm;

use cortex_m_rt::entry;


#[entry]
fn main() -> !{
    let mcu_peripherals = pac::Peripherals::take().unwrap();
    let mut rcc = mcu_peripherals.RCC.constrain();

    let mut flash = mcu_peripherals.FLASH.constrain();
    // let mut gpioa = mcu_peripherals.GPIO.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    
    // initialize user leds
    let mut gpioe = mcu_peripherals.GPIOE.split(&mut rcc.ahb);
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