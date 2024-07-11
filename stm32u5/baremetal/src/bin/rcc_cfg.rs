use stm32_metapac::rcc::{
    vals::{Plldiv, Pllm, Plln, Pllrge, Pllsrc, Sw},
    Rcc,
};

pub(crate) fn cfg_hsi(rcc: &mut Rcc) -> u16 {
    let rcc_cal = rcc.icscr3().read().hsical(); // why does this return u16 when its 8 bits long

    // wait for hsi to be ready
    // while !rcc.cr().read().hsirdy() {}

    rcc.cr().write(|w| w.set_hsion(true));

    rcc_cal
}

pub(crate) fn cfg_pll(rcc: &mut Rcc) {
    rcc.pll1cfgr().write(|w| {
        w.set_pllsrc(Pllsrc::HSI);
        w.set_pllrge(Pllrge::FREQ_8TO16MHZ);
        w.set_pllm(Pllm::DIV1);
    });
    rcc.pll1divr().write(|w| {
        w.set_plln(Plln::MUL10);
        w.set_pllr(Plldiv::DIV1);
    });
    rcc.pll1cfgr().write(|w| w.set_pllren(true));
    rcc.cfgr1().write(|w| {
        w.set_sw(Sw::PLL1_R);
    });
}

pub(crate) fn cfg_sys_clk(rcc: &mut Rcc) {}
