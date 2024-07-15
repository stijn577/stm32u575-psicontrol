#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! log {
    ($logger:expr) => {
        $logger
    };
}

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! log {
    ($logger:expr) => {};
}

/// A macro to benchmark the execution time of a function.
///
/// # Parameters
///
/// * `$fn`: The function to be benchmarked.
/// * `$freq`: The expected frequency of the function in Hz.
///
/// # Returns
///
/// This macro does not return any value. It logs the execution time of the function in seconds and microseconds.
///
/// # Example
///
/// ```rust
/// qbench!(led.set_high(), 4_000_000);
/// ```
/// this would then output: something like
/// ```bash
/// [ 140.236815 INFO src/main.rs:26  ] "led.set_high()" took 7.25e-6s = 7.25us
/// ```
#[macro_export]
macro_rules! qbench {
    ($fn: expr) => {{
        use defmt::info;
        use embassy_time::{Instant, TICK_HZ};
        use $crate::log;

        let now = Instant::now();
        let res = $fn;
        let diff = now.elapsed();
        let us = diff.as_ticks() as f64 * 1_000_000.0 / TICK_HZ as f64;
        log!(info!("{:?} took {:?}us", stringify!($fn), us));
        res
    }};
}
