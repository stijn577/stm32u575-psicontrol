#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;

use panic_probe as _;
use stm32_metapac::{adc::Adc, ADC4};

// docs: https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-armbased-32bit-mcus-stmicroelectronics.pdf#page=1363&zoom=100,165,224
// register map: p.1430
#[entry]
fn main() -> ! {
    let mut adc4 = ADC4;

    adc_vreg_en(&mut adc4);
    let calibration_factor = adc_cal(&mut adc4);
    adc_en(&mut adc4);
    adc_set_clk_psc(&mut adc4);
    adc_cfg(&mut adc4);

    loop {}
}

// p. 1372
fn adc_vreg_en(adc4: &mut Adc) {
    adc4.adc_isr().write(|w| w.set_ldordy(true));
    adc4.adc_cr().write(|w| w.set_advregen(true));

    while !adc4.adc_isr().read().ldordy() {
        asm::nop();
    }
}

// p. 1372
fn adc_cal(adc4: &mut Adc) -> usize {
    adc4.adc_pwr().write(|w| w.set_autoff(false));

    assert!(!adc4.adc_cr().read().aden());
    assert!(adc4.adc_cr().read().advregen());
    assert!(adc4.adc_isr().read().ldordy());
    assert!(!adc4.adc_pwr().read().autoff());
    assert!(!adc4.adc_cfgr1().read().dmaen());

    adc4.adc_cr().write(|w| w.set_adcal(true));

    while adc4.adc_cr().read().adcal() {
        asm::nop();
    }

    adc4.adc_calfact().read().calfact().into()
}

// p. 1374
fn adc_en(adc4: &mut Adc) {
    adc4.adc_isr().write(|w| w.set_adrdy(true));
    adc4.adc_cr().write(|w| w.set_aden(true));

    while adc4.adc_isr().read().adrdy() {
        asm::nop();
    }
}

fn adc_set_clk_psc(adc4: &mut Adc) {
    adc4.adc_ccr().write(|w| w.set_presc(10))
}

fn adc_cfg(adc4: &mut Adc) {
    adc4.adc_chselr().write(|w| w.set_chsel11(true));
}
