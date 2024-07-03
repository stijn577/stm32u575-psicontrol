#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_time::{Instant, Timer};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Level, Output, Pull, Speed},
    peripherals::{PC13, PC7},
};
use tasks::blinky::blinky;

mod tasks;
#[macro_use]
mod macros;

#[embassy_executor::main]
async fn main(_s: Spawner) {
    let pp = embassy_stm32::init(Default::default());

    info!("Hello world");

    let led = Output::new(pp.PC7, Level::Low, Speed::Low);
    // Warning:
    // The PC13 I/O used for the user button must be set to INPUT, pull‑down (PD) with
    // debouncing. Never set the PC13 to OUTPUT/LOW level to avoid a shortcut when the user
    // button is pressed.
    let btn = ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13);

    qbench!(functions::add(1, 2), 4_000_000);

    _s.spawn(blinky(btn, led)).ok();
}

