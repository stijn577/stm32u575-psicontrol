use embassy_stm32::{
    exti::ExtiInput,
    gpio::Output,
    peripherals::{PC13, PC7},
};
use embassy_time::Timer;

use crate::qbench;

#[embassy_executor::task]
pub async fn blinky(mut btn: ExtiInput<'static, PC13>, mut led: Output<'static, PC7>) {
    loop {
        qbench!(led.set_high(), 4_000_000);

        Timer::after_secs(1).await;

        qbench!(led.set_low(), 4_000_000);

        Timer::after_secs(1).await;
        
        qbench!(led.set_low(), 4_000_000);

        Timer::after_secs(1).await;

        qbench!(led.toggle(), 4_000_000);

        Timer::after_secs(1).await;
    }
}
