use defmt::debug;
use embassy_executor::task;
use embassy_stm32::exti::ExtiInput;
use setup::typedefs::Led;

#[task]
pub async fn btn_interrupt(mut btn: ExtiInput<'static>, mut led: Led) -> ! {
    loop {
        btn.wait_for_rising_edge().await;
        debug!("Button pressed!");
        led.set_high();

        btn.wait_for_falling_edge().await;
        debug!("Button released!");
        led.set_low();
    }
}
