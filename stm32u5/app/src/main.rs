#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::interrupt;
use setup::Board;
use tasks::{btn_interrupt::btn_interrupt, pwm::pwm_gen, uart::uart_rx};

// use defmt_rtt as _;
use panic_probe as _;

#[macro_use]
mod macros;
mod tasks;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = Board::init();

    s.spawn(uart_rx(board.usart1)).expect("Failed to start task");
    s.spawn(btn_interrupt(board.btn, board.led)).expect("Failed to start task");
    s.spawn(pwm_gen(board.pwm)).expect("Failed to start task");
}
