#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::{mux::Iclksel, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    Config,
};
// use setup::{typedefs::Led, Board};

use defmt_rtt as _;
// use functions::qbench;
use panic_probe as _;
use stm32_metapac::{gpio::Gpio, GPIOC};

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
        // config.rcc.mux.iclksel = Iclksel::HSI48;
    }

    let pp = embassy_stm32::init(config);

    // use this to set the configurations so the pin can work as an output pin
    // don't use let _ = ... to disable the unused return value, it will cause the warning to be fixed
    // but it will actually be optimized out completely -> pin isn't configured as output
    // let _out = Output::new(pp.PC12, Level::High, Speed::VeryHigh);
    GPIOC.moder().write(|w| w.set_moder(12, stm32_metapac::gpio::vals::Moder::OUTPUT));
    GPIOC.ospeedr().write(|w| w.set_ospeedr(12, stm32_metapac::gpio::vals::Ospeedr::VERYHIGHSPEED));

    loop {
        GPIOC.bsrr().write(|w| {
            w.set_bs(12, true);
        });
        GPIOC.bsrr().write(|w| {
            w.set_br(12, true);
        });
    }
}
