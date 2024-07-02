#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_time::{Instant, Timer};
use panic_probe as _;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Level, Output, Pull, Speed},
    peripherals::{PC13, PC7},
};


/// A macro to benchmark the execution time of a function.
///
/// # Parameters
///
/// * `$fn`: The function to be benchmarked.
/// * `$freq`: The expected frequency of the function in Hz.
///
/// # Returns
///
/// This macro does not return any value. It logs the execution time of the function in seconds and microseconds.
///
/// # Example
///
/// ```rust
/// qbench!(led.set_high(), 4_000_000);
/// ```
/// this would then output: something like
/// ```bash
/// [ 140.236815 INFO src/main.rs:26  ] "led.set_high()" took 7.25e-6s = 7.25us
/// ```
macro_rules! qbench {
    ($fn: expr, $freq: literal) => {{
        let before = Instant::now();
        $fn;
        let after = Instant::now();
        let diff = after - before;
        let s = diff.as_ticks() as f64 / $freq as f64;
        let us = s * 1_000_000.0;
        info!("{:?} took {:?}s = {:?}us", stringify!($fn), s, us)
    }};
}

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

    _s.spawn(blinky(btn, led)).ok();
}

#[embassy_executor::task]
async fn blinky(mut btn: ExtiInput<'static, PC13>, mut led: Output<'static, PC7>) {
    loop {
        qbench!(led.set_high(), 4_000_000);

        Timer::after_secs(1).await;

        qbench!(led.set_low(), 4_000_000);

        Timer::after_secs(1).await;

        qbench!(led.toggle(), 4_000_000);

        Timer::after_secs(1).await;

        qbench!(led.toggle(), 4_000_000);

        Timer::after_secs(1).await;
    }
}
