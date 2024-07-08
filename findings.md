# Findings

## 1. binary size

The following binary sizes where found, show in the following format: .text/.data/.bss

|                                                        Task | Rust optimized for size | C optimized for size | Rust optimized for speed | C optimized for speed |
| ----------------------------------------------------------: | :---------------------- | -------------------- | ------------------------ | --------------------- |
|    [Set pin high/low](./stm32u5/app/src/bin/toggle_fast.rs) | 11808/8/33108           | 6336/12/1572         | 13376/8/33100            | 7048/12/1572          |
| [Set pin raw reg](./stm32u5/app/src/bin/toggle_fast_raw.rs) | 10132/8/320             | 6320/12/1572         | 11028/8/324              | 7032/12/1572          |
|      [polling button](./stm32u5/app/src/bin/btn_polling.rs) | 11864/8/33096           | 6372/12/1572         | 13432/8/33100            | 7096/12/1572          |
|       [exti button](./stm32u5/app/src/bin/btn_interrupt.rs) | 12560/8/33096           | 7880/12/1644         | 14340/8/33100            | 8704/12/1644          |

Both were compiled with plenty of flags, the barebon amount of peripherals were used and the system clock was running at max speed (160MHz). Notice the Rust code is quite large in the .bss section. The reason for this is because embassy is a no_alloc framework. In order to avoid needing heap allocations for the tasks the tasks are stack allocated at compile time (and initialized later). The raw register version of the toggle fast binary doesn't really need the executor, so we can omit it, this is why its .bss section is much smaller, in the other cases it doesn't make much sense to work without the executor, since you will basically always be using it when using embassy-stm32.

The rust .text section still seems to do double the size for these small examples. A major part of the .text section is allocated to the init_pll function from embassy_stm32. This can be checked using cargo-bloat to see what is inflating the binary size it gives this kind of output. More interstingly, the executor itself has a rather small footprint.

## 2. performance

|                                                        Task |       data | Rust opt for size | C opt for size | Rust opt for speed | C opt for speed |
| ----------------------------------------------------------: | ---------: | ----------------- | -------------- | ------------------ | --------------- |
|    [Set pin high/low](./stm32u5/app/src/bin/toggle_fast.rs) | freq (xHz) | X                 | X              | X                  | X               |
| [Set pin raw reg](./stm32u5/app/src/bin/toggle_fast_raw.rs) | freq (xHz) | X                 | X              | X                  | X               |
|      [polling button](./stm32u5/app/src/bin/btn_polling.rs) |        ... | X                 | X              | X                  | X               |
|       [exti button](./stm32u5/app/src/bin/btn_interrupt.rs) | delay (xs) | X                 | X              | X                  | X               |
