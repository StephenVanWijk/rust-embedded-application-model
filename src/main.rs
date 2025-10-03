// CompassTrail_KanbanDPE_20250929_0910CET_SDvW


#![no_std]
#![no_main]
// Scaffolding 20250529_0910
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use panic_semihosting as _;
use cortex_m_semihosting::{hprint, hprintln};
use cortex_m_rt::entry;
use stm32f3_discovery::wait_for_interrupt;
use stm32f3_discovery::stm32f3xx_hal::{self as hal, pac, prelude::*, spi::Spi};
 
use core::{convert::TryInto, ops::Range};
use cortex_m::asm;

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x78;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let flash = dp.FLASH.constrain();

    loop {
        wait_for_interrupt();
    }
}