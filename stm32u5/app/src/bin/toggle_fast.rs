#![no_std]
#![no_main]

use embassy_executor::{task, Spawner};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::{mux::Iclksel, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    Config,
};

use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let mut config = Config::default();
    config.rcc.hsi = true;
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: None,
        divq: None,
        divr: Some(PllDiv::DIV1),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.voltage_range = VoltageScale::RANGE1;
    config.rcc.mux.iclksel = Iclksel::HSI48;

    let pp = embassy_stm32::init(config);

    let led = Output::new(pp.PC7, Level::Low, Speed::VeryHigh);

    s.spawn(toggle_fast(led)).expect("Failed to spawn task");
}

#[task]
pub async fn toggle_fast(mut led: Output<'static>) {
    loop {
        led.set_high();
        led.set_low();
    }
}
