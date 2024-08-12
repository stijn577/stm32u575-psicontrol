use defmt::info;
use embassy_executor::task;
use embassy_stm32::timer::Channel;
use embassy_time::Timer;
// use functions::pwm::PwmDuty;
use setup::typedefs::Pwm;

#[task]
pub async fn pwm_gen(mut pwm: Pwm) -> ! {
    let max = pwm.get_max_duty();
    info!("max duty cycle count: {:?}", max);

    pwm.enable(Channel::Ch2);

    loop {
        for i in 0..=max {
            pwm.set_duty(Channel::Ch2, i);
            Timer::after_millis(1).await;
        }
        for i in 0..=max {
            pwm.set_duty(Channel::Ch2, max - i);
            Timer::after_millis(1).await;
        }
    }
}

// #[task]
// pub async fn pwm_percent(mut pwm: Pwm) -> ! {
//     let pwmduty = PwmDuty::new(pwm.get_max_duty());
//     pwm.enable(Channel::Ch2);

//     loop {
//         // get the value for our specific percentage, if our percentage is not between 0 and 100,
//         // a None is returned and we can choose how to handle this "error" case, in this case we just
//         // provide a default value with the unwrap_or() function
//         for percent in 0..=100 {
//             let ticks = pwmduty.calc(percent).unwrap_or(0);
//             pwm.set_duty(Channel::Ch2, ticks);
//             Timer::after_millis(1).await;
//         }
//     }
// }
