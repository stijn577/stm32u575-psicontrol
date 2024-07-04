#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use setup::Board;
use tasks::{
    btn_interrupt::btn_interrupt, btn_polling::btn_polling, pwm::pwm_gen, toggle_fast::toggle_fast,
    uart::uart_rx,
};

use defmt_rtt as _;
use panic_probe as _;

#[macro_use]
mod macros;
mod tasks;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = Board::init();

    info!("Board initialized");

    // s.spawn(uart_rx(board.usart1)).expect("Failed to start task");
    // s.spawn(btn_polling(board.btn, board.led))
    // .expect("Failed to start task");
    // s.spawn(pwm_gen(board.pwm)).expect("Failed to start task");
    s.spawn(toggle_fast(board.led)).expect("Failed to start task");

    info!("Tasks spawned");
}
