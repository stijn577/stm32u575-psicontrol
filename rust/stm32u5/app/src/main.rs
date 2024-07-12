#![no_std]
#![no_main]

use embassy_executor::Spawner;
use setup::Board;
use tasks::{btn_interrupt::btn_interrupt, pwm::pwm_gen, spi::spi_comm, uart::uart_rx};

// use defmt_rtt as _;
use panic_probe as _;

#[macro_use]
mod macros;
mod tasks;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = qbench!(Board::init(), 160_000_000);

    s.spawn(uart_rx(board.usart1)).expect("Failed to start task");
    s.spawn(btn_interrupt(board.btn, board.led)).expect("Failed to start task");
    s.spawn(pwm_gen(board.pwm)).expect("Failed to start task");
    s.spawn(spi_comm(board.spi2)).expect("Failed to start task");
}
