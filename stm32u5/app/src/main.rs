#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use defmt::info;
use embassy_executor::Spawner;
use setup::Board;
use tasks::{uart::uart_rx, wfi_btn_set_led::wfi_btn_set_led};

mod tasks;
#[macro_use]
mod macros;

#[embassy_executor::main]
async fn main(_s: Spawner) {

    info!("Hello world");

    let board = Board::init();

    _s.spawn(uart_rx(board.usart1)).ok();
    _s.spawn(wfi_btn_set_led(board.btn, board.led)).ok();
}
