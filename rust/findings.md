# Findings

## 1. binary size

The following binary sizes where found, show in the following format: .text/.data/.bss

|                                                             Task | Rust optimized for size | C optimized for size | Rust optimized for speed | C optimized for speed |
| ---------------------------------------------------------------: | :---------------------- | -------------------- | ------------------------ | --------------------- |
|    [Set pin high/low](./rust/stm32u5/app/src/bin/toggle_fast.rs) | 11808/8/33108           | 6336/12/1572         | 13376/8/33100            | 7048/12/1572          |
| [Set pin raw reg](./rust/stm32u5/app/src/bin/toggle_fast_raw.rs) | 10132/8/320             | 6320/12/1572         | 11028/8/324              | 7032/12/1572          |
|      [polling button](./rust/stm32u5/app/src/bin/btn_polling.rs) | 11864/8/33096           | 6372/12/1572         | 13432/8/33100            | 7096/12/1572          |
|       [exti button](./rust/stm32u5/app/src/bin/btn_interrupt.rs) | 12560/8/33096           | 7856/12/1644         | 14340/8/33100            | 8680/12/1644          |

Both were compiled with plenty of flags, the barebon amount of peripherals were used and the system clock was running at max speed (160MHz). Notice the Rust code is quite large in the .bss section. The reason for this is because embassy is a no_alloc framework. In order to avoid needing heap allocations for the tasks the space required for tasks is allocated at compile time. We can configure how much space should be reserved (=#tasks) by setting the feature "task-arena-size-32768" for the embassy-executor crate. It configures the The raw register version of the toggle fast binary doesn't really need the executor, so we can omit it, this is why its .bss section is much smaller, in the other cases it doesn't make much sense to work without the executor, since you will basically always be using it when using embassy-stm32. The rust .text section still seems to do double the size for these small examples. A major part of the .text section is allocated to the init_pll function from embassy_stm32. This can be checked using cargo-bloat to see what is inflating the binary size it gives this kind of output. More interstingly, the executor itself has a rather small footprint.

## 2. Boot duration (time to initialize peripherals)

TODO: doublecheck these numbers cus that doesn't seem right

|                                                             Task | Rust   | C               |
| ---------------------------------------------------------------: | ------ | --------------- |
|    [Set pin high/low](./rust/stm32u5/app/src/bin/toggle_fast.rs) | 0.5625 | 14.15625 (2265) |
| [Set pin raw reg](./rust/stm32u5/app/src/bin/toggle_fast_raw.rs) | 0.875  | 14.15625 (2265) |
|      [polling button](./rust/stm32u5/app/src/bin/btn_polling.rs) | 1.0625 | 16.4625 (2634)  |
|       [exti button](./rust/stm32u5/app/src/bin/btn_interrupt.rs) | 1.0625 | 21.11875 (3379) |

In C, the measurement was done with the DWT Cycle Counter. Taking the difference between 2 values loaded before the initialisation starts and one after. Dividing this by the MCU core frequency will yield the duration. The code looks like this:

```c
uint32_t begin;
uint32_t end;

int main () {
    begin = DWT->CCYNT;
    // initialisation here
    end = DWT->CCYNT;
    uint32_t diff = end - begin;
}
```

We can then do the division manually. Note that Systick wasn't used because it might be a bit too inaccurate with it's 1kHz frequency. In Rust, embassy provides the Instant::now() function to get timestamps. So we can easily create a macro that takes a expression (for example a function) and a clock frequency as a literal. The invocation of this macro looks like this, wher eBoard::init() is a struct that exposes all peripherals that need to be initialised:

```rust
  let board = qbench!(Board::init());
```

The macro itself looks like this:

```rust
#[macro_export]
macro_rules! qbench {
    ($fn: expr) => {{
        use defmt::info;
        use embassy_time::{Instant, TICK_HZ};
        use $crate::log;

        let before = Instant::now();
        let res = $fn;
        let after = Instant::now();
        let diff = after - before;
        let us = diff.as_ticks() as f64 * 1_000_000.0 / TICK_HZ as f64;
        log!(info!("{:?} took {:?}us", stringify!($fn), us));
        res
    }};
}
```

It also allows us to return the values returned by the `$fn` expression, making it very flexible to take measurements. Optionally, logging can also be added in the macro to get it printed to the console. This is easier than having to look into the registers. The output of this logging looks like this (this is a debug build, so it normally doesn't take that long to initialise):

```bash
[ 0.00026900 INFO  app/src/main.rs:16 ] "Board::init()" took 241.625us
```

## 3. performance

|                                                             Task |       data | Rust opt for size | C opt for size | Rust opt for speed | C opt for speed |
| ---------------------------------------------------------------: | ---------: | ----------------- | -------------- | ------------------ | --------------- |
|    [Set pin high/low](./rust/stm32u5/app/src/bin/toggle_fast.rs) | freq (xHz) | X                 | X              | X                  | X               |
| [Set pin raw reg](./rust/stm32u5/app/src/bin/toggle_fast_raw.rs) | freq (xHz) | X                 | X              | X                  | X               |
|      [polling button](./rust/stm32u5/app/src/bin/btn_polling.rs) |        ... | X                 | X              | X                  | X               |
|       [exti button](./rust/stm32u5/app/src/bin/btn_interrupt.rs) | delay (xs) | X                 | X              | X                  | X               |
