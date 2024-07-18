#![no_std]
#![no_main]

// use defmt::info;
use embassy_executor::{task, Spawner};
use embassy_stm32::{
    can::Timestamp, gpio::{Level, Output, Speed}, rcc::{mux::Iclksel, AHBPrescaler, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale}, Config
};
use embassy_time::Timer;
use functions::qbench;
use setup::typedefs::Led;

// use defmt_rtt as _;
use panic_probe as _;
use stm32_metapac::lptim::vals::Presc;

#[embassy_executor::main]
async fn main(s: Spawner) {
    // let board = Board::init();
        let mut config = Config::default();
        {
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
        }
        let pp = embassy_stm32::init(config);

    // info!("Clocks configured");

    let mut led =  Output::new(pp.PC12, Level::Low, Speed::VeryHigh);

    loop {
        led.set_high();
        led.set_low();
    }
    
    // s.spawn(toggle_fast(led)).expect("Failed to spawn task");
}

#[task]
pub async fn toggle_fast(mut led: Led) {
    loop {
        led.set_high();
        led.set_low();
    }
}
