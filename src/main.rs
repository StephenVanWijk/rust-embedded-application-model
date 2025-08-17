#![no_std]
#![no_main]

use cortex_m::iprintln;
use cortex_m_rt::entry;
use rust_embedded_application_model::init;

#[entry]
fn main() -> ! {
    let mut itm = init();
    iprintln!(&mut itm.stim[0], "Hello, world!");
    
    loop {}
}