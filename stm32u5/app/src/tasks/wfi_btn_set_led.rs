use defmt::info;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::Output,
    peripherals::{PC13, PC7},
};

// use crate::qbench;

#[embassy_executor::task]
pub async fn wfi_btn_set_led(mut btn: ExtiInput<'static, PC13>, mut led: Output<'static, PC7>) {
    loop {
        btn.wait_for_rising_edge().await;
        info!("Button pressed");
        led.set_high();

        btn.wait_for_falling_edge().await;
        info!("Button released");
        led.set_low();
    }
}
