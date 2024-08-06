//! setup crate to be used in target-tests and app to initialize the device
//! no other functionality should go here.

#![no_std]
#![no_main]

use embassy_stm32::spi::{self, Spi};
use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Output, OutputType, Pull, Speed},
    peripherals::USART1,
    rcc::{Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    time::Hertz,
    timer::{
        low_level::CountingMode,
        simple_pwm::{PwmPin, SimplePwm},
    },
    usart::Uart,
    Config,
};
use typedefs::{Btn, Led, Pwm, Spi2, Uart1};

pub mod typedefs;

#[cfg(feature = "irqs")]
bind_interrupts!(pub struct Irqs {
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
    /// The Spi2 instance of the boar, to communicate over spi
    pub spi2: Spi2,
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
    #[cfg(feature = "init")]
    pub fn init() -> Board {
        let mut config = Config::default();

        Self::_clock_config(&mut config);

        let cp = cortex_m::Peripherals::take().unwrap(); // take core peripherals
        let dp = embassy_stm32::init(config); // take device peripherals

        let mut scb = cp.SCB;
        scb.enable_icache();
        let mut cpuid = cp.CPUID;
        scb.enable_dcache(&mut cpuid);

        let led = Output::new(dp.PC7, embassy_stm32::gpio::Level::Low, Speed::VeryHigh);
        // debug!("LED initialized");

        // Warning:
        // The PC13 I/O used for the user button must be set to INPUT, pull‑down (PD) with
        // debouncing. Never set the PC13 to OUTPUT/LOW level to avoid a shortcut when the user
        // button is pressed.
        let btn = ExtiInput::new(dp.PC13, dp.EXTI13, Pull::Down);
        // debug!("Button initialized");

        // it's ok to expect() here, because the device would not be initialized correctly if this fails
        let usart1 = Uart::new(dp.USART1, dp.PA10, dp.PA9, Irqs, dp.GPDMA1_CH10, dp.GPDMA1_CH11, Default::default()).expect("Failed to initialize USART1");
        // debug!("USART1 initialized");

        let pwm = SimplePwm::new(
            dp.TIM4,
            None,
            Some(PwmPin::new_ch2(dp.PB7, OutputType::PushPull)),
            None,
            None,
            Hertz::khz(160),
            CountingMode::EdgeAlignedUp,
        );
        // debug!("PWM initialized");

        let spi_cfg = spi::Config::default();
        let spi2 = Spi::new(dp.SPI2, dp.PB10, dp.PC1, dp.PC2, dp.GPDMA1_CH12, dp.GPDMA1_CH13, spi_cfg);

        Self { led, btn, usart1, pwm, spi2 }
    }

    fn _clock_config(config: &mut Config) {
        // just a helpful warning when someone did some stuff with the clocks
        // // debug!("Do not use HSE, it is not populated on the board");

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
        // config.rcc.mux.iclksel = Iclksel::HSI48;
    }
}
