# Tooling for embedded rust

## 1. Essentials

### 1. Compiler version and toolchain manager: install rustup ([see install guide rustlang.org](https://www.rust-lang.org/tools/install))

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install compiler toolchain for stm32u575 (with hardware fp unit)

```bash
rustup target add thumbv8m.main-none-eabihf
```

We can set the project to build to this target by default without having to pass `--target thumbv8em.main-none-eabihf` to the cargo run command every time. This can be set within the `.cargo/config.toml` file like so:

```toml
# .cargo/config.toml
[build]
target = "thumbv8em.main-none-eabihf"
```

### 3. Flashing code to the device: install probe-rs (flashing and debugging driver)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Other flashing tools can be used as well, binary is in target directory, we can set probe-rs in the `.cargo/config.toml` file to make it so `cargo run` invokes probe-rs to flash the binary. we can do that like this:

```toml
# .cargo/config.toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = [
    "probe-rs",
    "run",
    "--chip",
    "STM32U575ZITxQ",
    "--log-format",
    "{[ {t:010} {L:bold:<5} {fff:white:<0}:{l:white:<0} ]} {s}",
]
```

You may have noticed the --log-format flag, this will be made clear later, but this is to interact with defmt, a logging library that makes logging a lot easier. But also more performant, by storing the strings on the host and not the target. The target only sends over indices to an array so to speak. (of course if you want to log runtime data you need to send it over like normal still.)

## 2. Addtional tools

### 1. cargo watch

```bash
cargo install cargo-watch
```

Cargo watch allows you to run any cargo command automatically when the files in scope change, so you don't have to manually enter the command to compile every time. A cargo watch command will look something like this:

```bash
cargo watch -q -c -x 'r --release --package app'
```

This command will run the following command every time the code changes by typing in the files. The -q and -c flags are configuration flags for the watch command, quiet will hide output from the watch command itself so only the output from the build and run command are shown. -c clears the screen everytime the task recommences, showing only the final run which makes it easier to read relevant messages.

```bash
cargo r --release --package app
```

### 2. cargo-binutils

Cargo binutils provides tooling for reading binary output sizes similar to arm-none-eabi-gcc tools etc. They are built on llvm however, but they are easier to use because they are integrated with cargo making selecting the correct binary much easier. Using the llvm-tools is probably also going to be slightly better because the Rust backend is built on LLVM. To get the size of a for example the app binary we can get run the following code (in the stm32u5 directory).

```bash
cargo size --release --bin app
```

It the outputs the following response:

|  text | data |   bss |   dec |  hex | filename |
| ----: | ---: | ----: | ----: | ---: | -------- |
| 14228 |    8 | 33100 | 47336 | b8e8 | app      |

### 3. cargo-bloat

Cargo bloat allows .text section analysis to see what functions are taking up a lot of .text memory. This can help when trying to optimize binary output sizes or library crates. It can be run with a command looking like this:

```bash
cargo bloat --release --bin toggle_fast_raw
```

It outputs the following response:

|  File |  .text |   Size | Crate Name                                                                                                                                   |
| ----: | -----: | -----: | :------------------------------------------------------------------------------------------------------------------------------------------- |
| 24.3% |  53.4% | 5.2KiB | embassy_stm32 embassy_stm32::rcc::\_version::init_pll                                                                                        |
|  9.6% |  21.1% | 2.0KiB | embassy_stm32 embassy_stm32::\_generated::<impl core::ops::arith::Div<stm32_metapac::rcc::vals::Plldiv> for embassy_stm32::time::Hertz>::div |
|  6.6% |  14.5% | 1.4KiB | toggle_fast_raw toggle_fast_raw::\_\_cortex_m_rt_main                                                                                        |
|  1.0% |   2.2% |   218B | embassy_stm32 TIM15                                                                                                                          |
|  0.8% |   1.8% |   174B | embassy_stm32 EXTI0                                                                                                                          |
|  0.7% |   1.5% |   148B | embassy_stm32 embassy_stm32::rcc::RccInfo::enable_and_reset_with_cs                                                                          |
|  0.6% |   1.4% |   140B | embassy_stm32 embassy_stm32::dma::gpdma::\<impl embassy_stm32::dma::AnyChannel>::on_irq                                                      |
|  0.5% |   1.1% |   108B | embassy_stm32 embassy_stm32::gpio::Flex::set_as_output                                                                                       |
|  0.3% |   0.6% |    62B | cortex_m_rt Reset                                                                                                                            |
|  0.2% |   0.4% |    40B | panic_probe rust_begin_unwind                                                                                                                |
|  0.1% |   0.2% |    24B | [Unknown] HardFaultTrampoline                                                                                                                |
|  0.1% |   0.2% |    18B | std core::option::unwrap_failed                                                                                                              |
|  0.0% |   0.1% |     8B | std core::panicking::panic                                                                                                                   |
|  0.0% |   0.1% |     8B | std core::panicking::panic_bounds_check                                                                                                      |
|  0.0% |   0.1% |     8B | std core::panicking::panic_fmt                                                                                                               |
|  0.0% |   0.1% |     8B | [Unknown] main                                                                                                                               |
|  0.0% |   0.1% |     6B | embassy_stm32 GPDMA1_CHANNEL0                                                                                                                |
|  0.0% |   0.1% |     6B | embassy_stm32 GPDMA1_CHANNEL1                                                                                                                |
|  0.0% |   0.1% |     6B | embassy_stm32 GPDMA1_CHANNEL10                                                                                                               |
|  0.0% |   0.1% |     6B | embassy_stm32 GPDMA1_CHANNEL11                                                                                                               |
|  0.4% |   0.8% |    78B | And 15 smaller methods. Use -n N to show more.                                                                                               |
| 45.5% | 100.0% | 9.7KiB | .text section size, the file size is 21.3KiB                                                                                                 |
