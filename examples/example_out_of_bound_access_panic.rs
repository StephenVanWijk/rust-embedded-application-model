#![no_main]
#![no_std]

// 20250907 1345CET SDvW
// See https://docs.rs/panic-itm/latest/panic_itm/
// for GDB commands, to handle panic log messages.

use panic_semihosting as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let xs = [0, 1, 2];
    let i = xs.len();
    let _y = xs[i]; // out of bounds access

    loop {}
}