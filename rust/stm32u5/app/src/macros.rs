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
/// ```txt
/// [ 140.236815 INFO src/main.rs:26  ] "led.set_high()" took 7.25e-6s = 7.25us
/// ```
// #[macro_export]
// macro_rules! qbench {
//     ($fn: expr, $freq: literal) => {{
//         use defmt::info;
//         use embassy_time::Instant;

//         let before = Instant::now();
//         let res = $fn;
//         let after = Instant::now();
//         let diff = after - before;
//         let s = diff.as_ticks() as f64 / $freq as f64;
//         let us = s * 1_000_000.0;
//         info!("{:?} took {:?}s = {:?}us", stringify!($fn), s, us);
//         res
//     }};
// }
