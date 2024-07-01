# First steps with stm32u5

## 1. Installation

### 1. install rustup  ([see install guide rustlang.org](https://www.rust-lang.org/tools/install))

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. install compiler toolchain for stm32u575 (with hardware fp unit)

```bash
rustup target add thumbv8m.main-none-eabihf
```

### 3. install probe-rs (flashing and debugging driver) 

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Other flashing tools can be used as well, binary is in target directory