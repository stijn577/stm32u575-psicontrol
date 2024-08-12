use defmt::info;
use embassy_executor::task;
use embassy_stm32::gpio::Output;
use embassy_time::Timer;

// normally you wouldnt't use the unsafe functions as such, but use them in more complex structs to abstract away a bunch of unsafe operations, you should try to abstract it away
// into your own structs, not call unsafe everywhere, the ffi with C will be where most bugs originate from, so try to abstract as much of it as possible
#[inline]
pub fn increment(v: u32) -> u32 {
    unsafe { cbinder_sys::increment(v) }
}

#[task]
pub async fn cbinder_example(mut led_red: Output<'static>) -> ! {
    let mut a = 0;
    loop {
        info!("a={:?}", a);
        Timer::after_secs(1).await;
        a = increment(a);
        led_red.toggle();
    }    
}
