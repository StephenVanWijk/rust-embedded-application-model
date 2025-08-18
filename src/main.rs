#![no_std]
#![no_main]

use panic_itm as _; // panic handler
use cortex_m_rt::entry;
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, stm32, gpio::GpioExt};

#[entry]
fn main() -> ! {
    // Get access to device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    
    // Initialize the GPIO ports
    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    
    // Configure PE8 (North LED) as output
    let mut led = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    
    loop {
        // Toggle LED state
        let _ = led.toggle();
        
        // Simple busy-wait delay
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}