#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_time::Instant;
use panic_probe as _;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Level, Output, Pull, Speed},
};

#[embassy_executor::main]
async fn main(_s: Spawner) {
    let pp = embassy_stm32::init(Default::default());

    info!("Hello world");

    let mut led = Output::new(pp.PC7, Level::Low, Speed::Low);
    // Warning:
    // The PC13 I/O used for the user button must be set to INPUT, pull‑down (PD) with
    // debouncing. Never set the PC13 to OUTPUT/LOW level to avoid a shortcut when the user
    // button is pressed.
    let mut btn = ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13);

    loop {
        info!("waiting for rising edge...");
        btn.wait_for_rising_edge().await;
        let before = Instant::now();
        led.set_high();
        let diff = Instant::now() - before; // calculate difference in ticks between before and after
        info!("{:?}us", (diff.as_ticks() / 4)); // scale ticks to microsecondes (see ticks_hz_x used for embassy_time dependency)

        info!("waiting for falling edge...");
        btn.wait_for_falling_edge().await;
        led.set_low();
    }
}
