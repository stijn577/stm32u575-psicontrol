use defmt::info;
use embassy_executor::task;
use embassy_time::Timer;
use setup::heap::heap_usage;

#[task]
pub async fn heap_info() -> ! {
    loop {
        let heap_vals = heap_usage();
        info!("Current heap estimates\nused: {:?}\nfree: {:?}", heap_vals.0, heap_vals.1);
        Timer::after_secs(3).await;
    }
}
