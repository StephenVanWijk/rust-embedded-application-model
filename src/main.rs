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
use panic_itm as _;
// Import the function to enter low-power sleep waiting for an interrupt.
use stm32f3_discovery::wait_for_interrupt; 

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
    let reset_and_clock_control = device_periphs.RCC.constrain();

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

        // Setup 1 second systick.
    let mut syst = core_periphs.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    loop{
        wait_for_interrupt();
    }
}