//! Testing on the target device itself, this should be reserved for hardware specific functions only, purely
//! functional stuff should be etest in the host-tests for faster iteration

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

fn fn_false() -> bool {
    false
}

#[defmt_test::tests]
mod tests {
    use crate::fn_false;

    #[init]
    fn init() {
        let pp = embassy_stm32::init(Default::default());
    }
    #[test]
    fn test_add() {
        defmt::assert_eq!(functions::add(1, 2), 3);
    }

    // #[test]
    // fn fail() {
    //     panic!("expected");
    // }
}
