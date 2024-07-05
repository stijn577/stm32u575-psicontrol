use defmt::info;
use embassy_executor::task;
use embassy_stm32::exti::ExtiInput;
use setup::typedefs::Led;

#[task]
pub async fn btn_interrupt(mut btn: ExtiInput<'static>, mut led: Led) {
    loop {
        btn.wait_for_rising_edge().await;
        info!("Button pressed!");
        led.set_high();

        btn.wait_for_falling_edge().await;
        info!("Button released!");
        led.set_low();
    }
}
