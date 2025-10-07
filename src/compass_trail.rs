#![no_std]
// Scaffolding macro's, makes coding easier.
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use stm32f3_discovery::stm32f3xx_hal::{pac, prelude::*};
use stm32f3_discovery::stm32f3xx_hal::spi::{Spi, Mode, Phase, Polarity};

pub fn compass_rose_init() {
    // Clock Setup
    // Enable GPIO clocks for LED pins.
}

pub fn magnetometer_communication_init() {
    // TODO, determine if you can make this function generic.
    
    // TODO, make these two function arguments, so that they are kept in the fn main()
    let mcu_peripherals = pac::Peripherals::take().unwrap();
    let mut flash = mcu_peripherals.FLASH.constrain();

    /*  
    Clock Setup
    Enable GPIO clocks for SPI pins
    I don't understand this, I think I need one clock for SPI,Enable SPI1 peripheral clock
    */

    // Reset and Clock Control made in a Rust object that is usable
    let mut rcc = mcu_peripherals.RCC.constrain();

    // Reset and Clock Control configuration, with the freeze method,
    // to make sure that Reset and Clock Control works with the clock configuration,
    // from the Access Control Register, in the RAM, Flash memory. 
    let clock_rcc = rcc.cfgr.freeze(&mut flash.acr);

    // General Purpose Input and Output port A with a mutable reference to Reset and,
    // Clock Control via Advanced High Performance Bus.
    let mut gpioa = mcu_peripherals.GPIOA.split(&mut rcc.ahb);
    
    // TODO To understand why the NSS (Negative Slave Select or, CS, Chips Select) isn't used in AI examples
    // TODO Select the magnetometer chip, LSM303AGR explicitly, so use pins PA4, PA5, PA6, PA7

    // SPI Communication Setup
    // The pins are configured for SPI1 (Alternate Function 5, which forms together with pins PA4,PA5,PA6,PA7 SPI)
    // So, PA4, PA5, PA6, PA7 has to be configured so that they function as,
    // SPI1_NSS, SPI1_SCK, SPI1_MISO, SPI1_MOSI
    let spi1_nss = gpioa.pa4.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let spi1_sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let spi1_miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let spi1_mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    
    // Configure SPI1 in master mode with appropriate clock polarity/phases)
    // TODO make sure that the polarity and the phase are rightly configured.
    let polarity_phase = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };
    // TODO make sure that the SPI1 Hz rate is right.
    let spi = 
        Spi::spi1(
            mcu_peripherals.SPI1,
            (spi1_sck, spi1_miso, spi1_mosi),
            polarity_phase,
            1_000_000.hz(),
            clock_rcc,
            &mut rcc.apb2,
        );

    // Magnetometer Initialization
    // Configure magnetometer via SPI writes:
    // Set output data rate and operating mode
    // Enable temperature sensor for calibration
    // Configure data ready interrupts
}