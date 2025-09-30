#![no_std]
#![no_main]
// Scaffolding macro's, makes coding easier.
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Define specific panic handler.
// E.g. use panic_semihosting as _;
// Or   use panic_itm as _;
// The examples can not be used simultaneously,
// because they both implement the Rust trait:
// panic-impl .
use panic_itm as _;
use cortex_m_rt::entry;
use stm32f3_discovery::wait_for_interrupt;

#[entry]
fn main() -> ! {
    loop {
        wait_for_interrupt();
    }
}