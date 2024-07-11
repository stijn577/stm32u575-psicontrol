use cortex_m::asm;
use stm32_metapac::adc::Adc;

// p. 1372
pub(crate) fn adc_vreg_en(adc4: &mut Adc) {
    adc4.isr().write(|w| w.set_ldordy(true));
    adc4.cr().write(|w| w.set_advregen(true));

    // while !adc4.isr().read().ldordy() {
    //     asm::nop();
    // }
}

// p. 1372
pub(crate) fn adc_cal(adc4: &mut Adc) -> usize {
    adc4.pwrr().write(|w| w.set_autoff(false));

    assert!(!adc4.cr().read().aden());
    assert!(adc4.cr().read().advregen());
    assert!(adc4.isr().read().ldordy());
    assert!(!adc4.pwrr().read().autoff());
    assert!(!adc4.cfgr1().read().dmaen());

    adc4.cr().write(|w| w.set_adcal(true));

    while adc4.cr().read().adcal() {
        asm::nop();
    }

    adc4.calfact().read().calfact().into()
}

// p. 1374
pub(crate) fn adc_en(adc4: &mut Adc) {
    adc4.isr().write(|w| w.set_adrdy(true));
    adc4.cr().write(|w| w.set_aden(true));

    while adc4.isr().read().adrdy() {
        asm::nop();
    }
}

pub(crate) fn adc_set_clk_psc(adc4: &mut Adc) {
    adc4.ccr().write(|w| w.set_presc(10));
}

pub(crate) fn adc_chan_sel(adc4: &mut Adc) {
    adc4.cfgr1().write(|w| w.set_chselrmod(false)); // technically not needed but for explicitness
    adc4.chselrmod0().write(|w| w.set_chsel(11)); // p. 1377, ADC4_IN11 => Vin,11
}

pub(crate) fn adc_start_conversion(adc4: &mut Adc) {
    adc4.cfgr1().write(|w| {
        w.set_cont(false);
        w.set_exten(0);
    });

    adc4.cr().write(|w| w.set_adstart(true));
}

pub(crate) fn adc_wait_done(adc4: &mut Adc) -> usize {
    while adc4.cr().read().adstart() {
        asm::nop();
    }

    adc4.dr().read().data().into()
}
