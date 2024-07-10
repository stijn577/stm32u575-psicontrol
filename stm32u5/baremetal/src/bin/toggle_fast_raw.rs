#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::{mux::Iclksel, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    Config,
};
// use setup::{typedefs::Led, Board};

// use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
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
        config.rcc.mux.iclksel = Iclksel::HSI48;
    }
   
    let pp = embassy_stm32::init(config);

    // use this to set the configurations so the pin can work as an output pin
    // don't use let _ = ... to mdisable the unused return value, it will cause the warning to be fixed
    // but it will actually be optimized out completely -> pin isn't configured as output
    let _led = Output::new(pp.PC7, Level::Low, Speed::VeryHigh);

    loop {
        stm32_metapac::GPIOC.bsrr().write(|w| w.set_bs(7, true));
        stm32_metapac::GPIOC.bsrr().write(|w| w.set_br(7, false));
    }
}
