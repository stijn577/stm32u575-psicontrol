//! Testing on the target device itself, this should be reserved for hardware specific functions only, purely
//! functional stuff should be tested in the host-tests for faster iteration. So reserve this for testing
//! certain sensors (this can be split into different test files in the tests/ directory).

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use setup::Board;

#[defmt_test::tests]
mod tests {

    use super::*;

    #[init]
    fn init() -> Board {
        let pp = embassy_stm32::init(Default::default());
        Board::init(pp)
    }

    #[test]
    fn test_add(_board: &mut Board) {
        defmt::assert_eq!(functions::add(1, 2), 3);
    }

    #[test]
    fn test_led_set_high(board: &mut Board) {
        let led = &mut board.led;

        led.set_high();

        defmt::assert!(true);
    }
}
