// STM32F303 PCB KanbanDPE 20250918 Day 2
// Operatie. Uitvoeren. Het compas van de STM32F303 programmeren.
// [x] Welke external MCU peripheral is nodig om het magnetisch noorden te meten?
//     Antwoord: The LSM303DLHC accelerometer and magnetometer.
// [x] Op welke port komen de signalen binnen van LSM303DLHC?
//    Antwoord:
//    SDA Serial DAta, MCU PB7;
//    SCL Serial CLock, MCU PB6.
// [] Hoe zet je die data van de peripheral om in een brandende LED, 
//    die naar het magnetisch noorden wijst?
// [] Zorg dat de data op ports PBA6 en PB7 uitlezen kan worden via GDB.

#![no_std]
#![no_main]

// 20250919 1026CET SDvW
// The next macros I use scaffolding, these warnings make it harder to read.
// While I build the code according the TriviumDPE pattern.
#![allow(unused_imports)]
#![allow(unused_variables)]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::accelerometer::RawAccelerometer; // Imports the trait providing the accel_raw() method.
use stm32f3_discovery::compass::Compass; // Imports the driver for the LSM303DLHC accelerometer and magnetometer.
use panic_itm as _;
use stm32f3_discovery::wait_for_interrupt; // Import the function to enter low-power sleep waiting for an interrupt. 

// Entry Point.
// Marks the following function as the Cortex-M program entry point,
// setting the initial stack pointer and reset vector.
#[entry] 
// Defines the entry function.
// The ! return type indicates it never returns.
fn main() -> !{
    // Takes exclusive ownership of all the device-specific peripherals (e.g., GPIO, I2C, RCC).
    let device_periphs = pac::Peripherals::take().unwrap(); 
    // Takes the RCC peripheral and applies the "constrain" pattern,
    // converting it into a struct that provides a safe interface for clock configuration.
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    // Takes exclusive ownership of the core peripherals (SysTick, NVIC, ITM)
    // provided by the Cortex-M processor.
    let core_periphs = cortex_m::Peripherals::take().unwrap();
    // Takes the FLASH peripheral and applies the "constrain"
    // pattern to configure its access latency.
    let mut flash = device_periphs.FLASH.constrain();
    // Finalizes the clock configuration (sets SYSCLK, HCLK, PCLK frequencies) 
    // and freezes the settings. The FLASH Access Control Register is adjusted 
    // to account for the new clock speed.
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    // Setup one second system tick.
    let mut syst = core_periphs.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    // GPIO and I2C Peripheral Configuration:
    // Splits the GPIOB peripheral into individual pins. The AHB bus register is passed to enable the GPIOB clock.
    let mut gpiob = device_periphs.GPIOB.split(&mut reset_and_clock_control.ahb);
    // Compass (LSM303DLHC) Driver Initialization:
    // Constructs a new Compass driver instance.
    let mut compass = Compass::new(
        gpiob.pb6, // Provides Pin PB6 for the I2C SCL line.
        gpiob.pb7, //Provides Pin PB7 for the I2C SDA line.
        &mut gpiob.moder, // Provides access to the GPIO mode register to set the pin alternate functions.
        &mut gpiob.afrl, // Provides access to the alternate function low register to select the I2C function for the pins.
        device_periphs.I2C1, // Provides the I2C1 peripheral.
        clocks, // Provides the clock configuration to compute I2C timing.
        &mut reset_and_clock_control.apb1) // Provides access to the APB1 bus register to enable the I2C1 clock.
        .unwrap(); // Unwraps the Result, panicking if I2C initialization fails.

    loop{
        let accel = compass.accel_raw().unwrap(); // Reads the raw accelerometer values (X, Y, Z) over I2C. Unwraps the Result, panicking on an I2C error.
        // cortex_m::asm::nop();
        wait_for_interrupt();
    }
}

#[exception] //- Attribute denoting this function as an exception (interrupt) handler for the SysTick exception.
fn SysTick() { //- Defines the handler function.
   cortex_m::asm::nop(); 
}//- A no-operation instruction. It ensures the handler is not optimized out and provides a minimal breakpoint location. The primary function of the interrupt is to wake the core from the WFI sleep in the main loop.