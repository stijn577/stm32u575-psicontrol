#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use setup::Board;
use tasks::{pwm::pwm_gen, uart::uart_rx, wfi_btn_set_led::exti_btn};

use defmt_rtt as _;
use panic_probe as _;

#[macro_use]
mod macros;
mod tasks;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = Board::init();

    info!("Board initialized");

    s.spawn(uart_rx(board.usart1)).expect("Failed to start task");
    s.spawn(exti_btn(board.btn, board.led)).expect("Failed to start task");
    s.spawn(pwm_gen(board.pwm)).expect("Failed to start task");

    info!("Tasks spawned");
}
