use defmt::debug;
use embassy_executor::task;
use embassy_stm32::exti::ExtiInput;
use setup::typedefs::LedGreen;

#[task]
pub async fn btn_interrupt(mut btn: ExtiInput<'static>, mut led_green: LedGreen) -> ! {
    loop {
        btn.wait_for_rising_edge().await;
        debug!("Button pressed!");
        led_green.set_high();

        btn.wait_for_falling_edge().await;
        debug!("Button released!");
        led_green.set_low();
    }
}
