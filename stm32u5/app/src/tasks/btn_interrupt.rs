use defmt::info;
use embassy_executor::task;
use embassy_stm32::{exti::ExtiInput, peripherals::PC13};
use setup::typedefs::{Btn, Led};

#[task]
pub async fn btn_interrupt(mut btn: ExtiInput<'static, PC13>, mut led: Led) {
    loop {
        btn.wait_for_rising_edge().await;
        info!("Button pressed");
        led.set_high();

        btn.wait_for_falling_edge().await;
        info!("Button released");
        led.set_low();
    }
}
