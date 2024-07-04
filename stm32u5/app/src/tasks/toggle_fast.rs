use embassy_executor::task;
use setup::typedefs::Led;

#[task]
pub async fn toggle_fast(mut led: Led) {
    loop {
        led.set_high();
        led.set_low();
    }
}
