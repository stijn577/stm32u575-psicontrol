use embassy_executor::task;
use setup::typedefs::{Btn, Led};

#[task]
pub async fn btn_polling(mut btn: Btn, mut led: Led) {
    loop {
        if btn.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
