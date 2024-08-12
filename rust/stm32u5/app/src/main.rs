#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use setup::Board;
use tasks::{btn_interrupt::btn_interrupt, cbinder::cbinder_example, heap_usage::heap_info, pwm::pwm_gen, spi::spi_comm, uart::uart_comm};

use defmt_rtt as _;
use panic_probe as _;

mod tasks;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = Board::init(65535);

    s.spawn(btn_interrupt(board.btn, board.led_green)).expect("Failed to start BTN + LED task");
    s.spawn(cbinder_example(board.led_red)).expect("Failed to start cbinder task");
    s.spawn(pwm_gen(board.pwm)).expect("Failed to start PWM task");
    s.spawn(spi_comm(board.spi2)).expect("Failed to start SPI task");
    s.spawn(uart_comm(board.usart1)).expect("Failed to start UART task");
    s.spawn(heap_info()).expect("Failed to start heap info task")
}
