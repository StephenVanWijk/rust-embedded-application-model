#![no_std]
#![no_main]
// Scaffolding macro's, makes coding easier.
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::stm32f3xx_hal::gpio::gpioa;
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, gpio::GpioExt, pac};
use stm32f3_discovery::switch_hal::{ToggleableOutputSwitch};
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3_discovery::wait_for_interrupt;

// Define specific panic handler.
// E.g. use panic_semihosting as _;
// Or use panic_itm as _;
// The examples can not be used simultaneously,
// because they both implement the Rust trait:
// panic-impl .
#[entry]
fn main() -> ! {
    let mcu_peripherals = pac::Peripherals::take().unwrap();
    let mut rcc = mcu_peripherals.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = mcu_peripherals.FLASH.constrain();
    
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = mcu_peripherals.GPIOA.split(&mut rcc.ahb);
    let spi1_nss = gpioa.pa4.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mut nss_output = spi1_nss.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    nss_output.set_low().ok();

    let spi1_sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let spi1_miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let spi1_mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    // Identify magnetometer device.
    loop {
        wait_for_interrupt();
    }
}