//! setup crate to be used in target-tests and app to initialize the device
//! no other functionality should go here.

#![no_std]
#![no_main]

use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Input, Output, Pull, Speed},
    peripherals::{PC13, PC7, USART1},
    usart::Uart,
};
use typedefs::Uart1;

pub mod typedefs;

bind_interrupts!(struct Irqs {
    USART1 => embassy_stm32::usart::InterruptHandler<USART1>;
}
);

/// A struct representing the board with LED and button components.
/// Fields are public so they can be moved out of the struct easily.
pub struct Board {
    /// The LED component of the board.
    /// It's an output pin that can be set to high or low.
    pub led: Output<'static, PC7>,

    /// The button component of the board.
    /// It's an input pin with an associated external interrupt.
    pub btn: ExtiInput<'static, PC13>,
    pub usart1: Uart1,
}

impl Board {
    /// Initializes and returns a new instance of the [`Board`] struct.
    ///
    /// # Arguments
    ///
    /// * `pp` - The [`Peripherals`] struct, which contains all the peripheral instances.
    ///
    /// # Returns
    ///
    /// * A new instance of the [`Board`] struct, containing the LED and button components.
    ///
    /// # Example
    ///
    /// ```rust
    /// use embassy_stm32::Peripherals;
    /// use setup::Board;
    ///
    /// let pp = embassy_stm32::init(Default::default());
    /// let board = Board::init(pp);
    /// let led = board.led; // led moved out of board here and can be used independently.
    /// ```
    pub fn init() -> Board {
        let pp = embassy_stm32::init(Default::default());

        Self {
            led: Output::new(pp.PC7, embassy_stm32::gpio::Level::Low, Speed::Low),
            // Warning:
            // The PC13 I/O used for the user button must be set to INPUT, pull‑down (PD) with
            // debouncing. Never set the PC13 to OUTPUT/LOW level to avoid a shortcut when the user
            // button is pressed.
            btn: ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13),
            usart1: Uart::new(
                pp.USART1,
                pp.PA10,
                pp.PA9,
                Irqs,
                pp.GPDMA1_CH10,
                pp.GPDMA1_CH11,
                Default::default(),
            )
            .expect("Failed to initialize USART1"),
        }
    }
}
