//! setup crate to be used in target-tests and app to initialize the device
//! no other functionality should go here.

#![no_std]
#![no_main]

use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Output, OutputType, Pull, Speed},
    peripherals::USART1,
    rcc::{mux::Iclksel, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    time::Hertz,
    timer::{
        low_level::CountingMode,
        simple_pwm::{PwmPin, SimplePwm},
    },
    usart::Uart,
    Config,
};
use typedefs::{Btn, Led, Pwm, Uart1};

#[cfg(feature = "defmt")]
use defmt::{debug, info};

pub mod typedefs;
#[macro_use]
mod macros;

bind_interrupts!(struct Irqs {
    USART1 => embassy_stm32::usart::InterruptHandler<USART1>;
});

/// A struct representing the board with LED and button components.
/// Fields are public so they can be moved out of the struct easily.
pub struct Board {
    /// GPIO output to drive the green LED
    pub led: Led,
    /// The blue button on the board.
    pub btn: Btn,
    /// The usart1 instance of the board, to communicate over uart
    pub usart1: Uart1,
    /// Pwm signal generator for driving the blue LED
    pub pwm: Pwm,
}

impl Board {
    /// Initializes and returns a new instance of the [`Board`] struct.
    ///
    /// # Returns
    ///
    /// * A new instance of the [`Board`] struct, containing the required peripherals.
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
        let mut config = Config::default();

        log!(info!("Initializing board..."));
        log!(info!("Configuring clocks..."));

        Self::_clock_config(&mut config);

        let pp = embassy_stm32::init(config);

        log!(info!("Configuring Peripherals..."));

        let led = Output::new(pp.PC7, embassy_stm32::gpio::Level::Low, Speed::VeryHigh);
        log!(debug!("Led initialized"));

        // Warning:
        // The PC13 I/O used for the user button must be set to INPUT, pull‑down (PD) with
        // debouncing. Never set the PC13 to OUTPUT/LOW level to avoid a shortcut when the user
        // button is pressed.
        let btn = ExtiInput::new(pp.PC13, pp.EXTI13, Pull::Down);
        log!(debug!("Button initialized"));

        // it's ok to expect() here, because the device would not be initialized correctly if this fails
        let usart1 = Uart::new(pp.USART1, pp.PA10, pp.PA9, Irqs, pp.GPDMA1_CH10, pp.GPDMA1_CH11, Default::default()).expect("Failed to initialize USART1");
        log!(debug!("Usart1 initialized"));

        let pwm = SimplePwm::new(
            pp.TIM4,
            None,
            Some(PwmPin::new_ch2(pp.PB7, OutputType::PushPull)),
            None,
            None,
            Hertz::khz(160),
            CountingMode::EdgeAlignedUp,
        );
        log!(debug!("Pwm initialized"));

        log!(info!("Board initialized successfully!"));

        Self { led, btn, usart1, pwm }
    }

    fn _clock_config(config: &mut Config) {
        // just a helpful warning when someone did some stuff with the clocks
        // debug!("Do not use HSE, it is not populated on the board");

        // use STM32CubeMX for clock config generation, than just copy paste the values
        config.rcc.hsi = true;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL10,
            divp: None,
            divq: None,
            divr: Some(PllDiv::DIV1),
        });
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.voltage_range = VoltageScale::RANGE1;
        config.rcc.mux.iclksel = Iclksel::HSI48;
    }
}
