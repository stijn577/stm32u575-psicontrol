use defmt::info;
use embassy_executor::task;
use setup::typedefs::{Btn, Led};

// use crate::qbench;

#[task]
pub async fn exti_btn(mut btn: Btn, mut led: Led) {
    loop {
        btn.wait_for_rising_edge().await;
        info!("Button pressed");
        led.set_high();

        btn.wait_for_falling_edge().await;
        info!("Button released");
        led.set_low();
    }
}
