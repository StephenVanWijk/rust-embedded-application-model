// STM32F303 PCB KanbanDPE 20250918 Day 2
// Operatie. Uitvoeren. Het compas van de STM32F303 programmeren.
#![no_std]
#![no_main]

extern crate panic_itm as _;
use cortex_m_rt::entry;

#[entry]
fn main()-> !{
    loop{}
}