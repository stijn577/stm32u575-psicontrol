use core::ops::{Deref, DerefMut};
use defmt::info;
use embassy_executor::task;
use embassy_stm32::timer::Channel;
use embassy_time::Timer;
use setup::typedefs::Pwm;

#[task]
pub async fn pwm_gen(mut pwm: Pwm) {
    defmt::info!("{:?}", pwm.get_max_duty());

    let pwm_duty = PwmDuty(pwm.get_max_duty());

    pwm.enable(Channel::Ch2);

    loop {
        pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(0).unwrap());
        Timer::after_millis(1000).await;

        pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(50).unwrap());
        Timer::after_millis(1000).await;

        pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(50).unwrap());
        Timer::after_millis(1000).await;

        pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(100).unwrap());
        Timer::after_millis(1000).await;
    }
}

struct PwmDuty(u16);

impl PwmDuty {
    fn calc_duty(&self, percent: u16) -> Result<u16, ()> {
        // early return error if value passed in is not readable as a percent
        if !(0..=100).contains(&percent) {
            return Err(());
        }

        Ok(self.0 / 100 * percent) // divide first, overflow detected and panics in debug, but not in release
    }
}
