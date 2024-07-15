#![no_std]
#![no_main]

use core::borrow::Borrow;

use cortex_m::peripheral::DWT;
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use functions::qbench;
use setup::Board;
use tasks::{btn_interrupt::btn_interrupt, pwm::pwm_gen, spi::spi_comm, uart::uart_comm};

use defmt_rtt as _;
use panic_probe as _;

mod tasks;

#[embassy_executor::main]
async fn main(s: Spawner) {
    let board = qbench!(Board::init());
    // let usart1 = qbench!({
    //     let mut config = Config::default();
    //     {
    //         config.rcc.hsi = true;
    //         config.rcc.pll1 = Some(Pll {
    //             source: PllSource::HSI,
    //             prediv: PllPreDiv::DIV1,
    //             mul: PllMul::MUL10,
    //             divp: None,
    //             divq: None,
    //             divr: Some(PllDiv::DIV1),
    //         });
    //         config.rcc.sys = Sysclk::PLL1_R;
    //         config.rcc.voltage_range = VoltageScale::RANGE1;
    //         config.rcc.mux.iclksel = Iclksel::HSI48;
    //     }        
    //     let pp = embassy_stm32::init(config);
    //      Uart::new(pp.USART1, pp.PA10, pp.PA9, Irqs, pp.GPDMA1_CH10, pp.GPDMA1_CH11, Default::default()).expect("Failed to initialize USART1")
    //     },    
    //     160_000_000
    // );

    let now = Instant::now();
    Timer::after_millis(1000).await;

    s.spawn(uart_comm(board.usart1)).expect("Failed to start task");
    s.spawn(btn_interrupt(board.btn, board.led)).expect("Failed to start task");
    s.spawn(pwm_gen(board.pwm)).expect("Failed to start task");
    s.spawn(spi_comm(board.spi2)).expect("Failed to start task");
}
