#![no_std]
#![no_main]

// 20250907 1354CET SDvW, KanbanDPE
// STM32F303 PCB KanbanDPE 20250906
// STM32F303 Rust Embedded Code 20250906 1208CET SDvW dag 2
//
// extern crate panic_itm; 
// Has the same effect as:
use panic_itm as _; 
// The prior line of code is more self documenting, 
// because _ tells the compiler and other developers
// that the crate is used implicitly.

use cortex_m_rt::entry;

use stm32f3_discovery::wait_for_interrupt;

use stm32f3_discovery::stm32f3xx_hal::interrupt;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;

use core::sync::atomic::{AtomicBool, Ordering};
use stm32f3_discovery::button;
use stm32f3_discovery::button::interrupt::TriggerMode;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::ToggleableOutputSwitch;

static USER_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn EXTI0() {
    //If we don't clear the interrupt to signal it's been serviced, it will continue to fire.
    button::interrupt::clear();
    // pa0 has a low pass filter on it, so no need to debounce in software
    USER_BUTTON_PRESSED.store(true, Ordering::Relaxed);
}

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

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
    let mut led_array = 
    [leds.ld3, leds.ld5, leds.ld7, leds.ld9, leds.ld10, leds.ld8, leds.ld6, leds.ld4];
    let mut led_status = [false; 8];
    // let mut status_led = led_array[0];

    button::interrupt::enable(
        &device_periphs.EXTI,
        &device_periphs.SYSCFG,
        TriggerMode::Rising,
    );

    let mut toggle_index: usize = 0;    
    loop {
        if USER_BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
        // Turn off previous LED if it was on
        if toggle_index > 0 && led_status[toggle_index - 1] {
            led_array[toggle_index - 1].toggle().ok();
            led_status[toggle_index - 1] = false;
        } else if toggle_index == 0 && led_status[7] {
            led_array[7].toggle().ok();
            led_status[7] = false;
        }

        // Toggle current LED and update status
        led_array[toggle_index].toggle().ok();
        led_status[toggle_index] = !led_status[toggle_index]; // Flip the state

        // Move to next index
        toggle_index = (toggle_index + 1) % 8;
        }
    wait_for_interrupt();
    }
}