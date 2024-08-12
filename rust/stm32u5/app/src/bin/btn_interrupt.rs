#![no_std]
#![no_main]


// use defmt::info;
use embassy_executor::task;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::{
    gpio::{Level, Output, Pull, Speed},
    rcc::{mux::Iclksel, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    Config,
};

use defmt_rtt as _;
// use functions::qbench;
use panic_probe as _;

#[embassy_executor::main]
async fn main(s: Spawner) {
     
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

        let btn = ExtiInput::new(pp.PC13, pp.EXTI13, Pull::Down); 
        let led =  Output::new(pp.PC12, Level::Low, Speed::VeryHigh);
    

    s.spawn(btn_interrupt(btn, led)).expect("Failed to start task");
}

#[task]
pub async fn btn_interrupt(mut btn: ExtiInput<'static>, mut led: Output<'static>) {
    loop {
        btn.wait_for_rising_edge().await;
        led.set_high();

        btn.wait_for_falling_edge().await;
        led.set_low();
    }
}
