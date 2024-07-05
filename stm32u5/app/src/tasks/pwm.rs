use embassy_executor::task;
use embassy_stm32::timer::Channel;
use embassy_time::Timer;
use setup::typedefs::Pwm;

#[task]
pub async fn pwm_gen(mut pwm: Pwm) {
    // defmt::info!("{:?}", pwm.get_max_duty());
    let max = pwm.get_max_duty();
    // let pwm_duty = PwmDuty::new(pwm.get_max_duty());

    pwm.enable(Channel::Ch2);

    loop {
        for i in 0..max {
            pwm.set_duty(Channel::Ch2, i);
            Timer::after_micros(500).await
        }
        for i in 0..max {
            pwm.set_duty(Channel::Ch2, max - i);
            Timer::after_micros(500).await;
        }

        // pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(0).unwrap());
        // Timer::after_millis(1000).await;

        // pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(50).unwrap());
        // Timer::after_millis(1000).await;

        // pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(50).unwrap());
        // Timer::after_millis(1000).await;

        // pwm.set_duty(Channel::Ch2, pwm_duty.calc_duty(100).unwrap());
        // Timer::after_millis(1000).await;
    }
}
