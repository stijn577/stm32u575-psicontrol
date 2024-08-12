#![no_std]
#![no_main]

use adc_cfg::{adc_cal, adc_chan_sel, adc_en, adc_set_clk_psc, adc_start_conversion, adc_vreg_en, adc_wait_done};
use cortex_m::asm;
use cortex_m_rt::entry;
// use rcc_cfg::{cfg_hsi, cfg_pll, cfg_sys_clk};
use stm32_metapac::{
    common::W,
    rcc::vals::{Adcdacsel, Pllm, Sw},
    ADC4, ADC4_COMMON, RCC,
};

use panic_probe as _;

mod adc_cfg;
mod rcc_cfg;

// docs: https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-armbased-32bit-mcus-stmicroelectronics.pdf#page=1363&zoom=100,165,224
// register map: p.1430
#[entry]
fn main() -> ! {
    // TODO: probably configure the core clocks etc to feed the ADC, see if that's the issue, things get stuck at while !adc4.adc_isr().read().ldordy()

    let mut adc4 = ADC4;
    let rcc = RCC;

    adc4.cr().write(|w| w.set_addis(true));
    rcc.cr().write(|w| w.set_hsion(true));
    while !rcc.cr().read().hsirdy() {
        asm::nop();
    }
    rcc.cfgr1().write(|w| {
        w.set_sw(Sw::HSI);
    });
    rcc.ccipr3().write(|w| w.set_adcdacsel(Adcdacsel::HSI));
    adc_set_clk_psc(&mut adc4);
    rcc.ahb3enr().write(|w| w.set_adc4en(true));
    adc_vreg_en(&mut adc4);
    adc4.cr().write(|w| w.set_aden(true));
    let _calibration_factor = adc_cal(&mut adc4);
    adc_en(&mut adc4);
    adc_chan_sel(&mut adc4);
    // ignore sample time for now, let's use default

    let mut _value;
    loop {
        adc_start_conversion(&mut adc4);
        _value = adc_wait_done(&mut adc4);
    }
}
