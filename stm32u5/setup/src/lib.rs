//! setup crate to be used in target-tests and app to initialize the device
//! no other functionality should go here.

#![no_std]
#![no_main]

use defmt::debug;
use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Input, Output, OutputType, Pull, Speed},
    peripherals::USART1,
    rcc::{
        self, AHBPrescaler, APBPrescaler, ClockSrc, Hsi48Config, LsConfig, PllConfig, RtcClockSource,
        VoltageScale,
    },
    time::Hertz,
    timer::{
        simple_pwm::{PwmPin, SimplePwm},
        CountingMode,
    },
    usart::Uart,
    Config,
};
use typedefs::{Btn, Led, Pwm, Uart1};

pub mod typedefs;

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

        {
            config.rcc = Self::clock_config();
        }

        let pp = embassy_stm32::init(config);

        let led = Output::new(pp.PC7, embassy_stm32::gpio::Level::Low, Speed::Low);
        debug!("LED initialized");

        // Warning:
        // The PC13 I/O used for the user button must be set to INPUT, pull‑down (PD) with
        // debouncing. Never set the PC13 to OUTPUT/LOW level to avoid a shortcut when the user
        // button is pressed.
        let btn = Input::new(pp.PC13, Pull::Down);
        debug!("Button initialized");

        // it's ok to expect() here, because the device would not be initialized correctly if this fails
        let usart1 = Uart::new(
            pp.USART1,
            pp.PA10,
            pp.PA9,
            Irqs,
            pp.GPDMA1_CH10,
            pp.GPDMA1_CH11,
            Default::default(),
        )
        .expect("Failed to initialize USART1");
        debug!("USART1 initialized");

        let pwm = SimplePwm::new(
            pp.TIM4,
            None,
            Some(PwmPin::new_ch2(pp.PB7, OutputType::PushPull)),
            None,
            None,
            Hertz::khz(160),
            CountingMode::EdgeAlignedUp,
        );
        debug!("PWM initialized");

        Self {
            led,
            btn,
            usart1,
            pwm,
        }
    }

    fn clock_config() -> rcc::Config {
        // use STM32CubeMX for clock config generation, than just copy paste the values
        rcc::Config {
            mux: ClockSrc::PLL1_R(PllConfig::msis_160mhz()),
            ahb_pre: AHBPrescaler::DIV1,
            apb1_pre: APBPrescaler::DIV1,
            apb2_pre: APBPrescaler::DIV1,
            apb3_pre: APBPrescaler::DIV1,
            hsi48: Some(Hsi48Config { sync_from_usb: false }),
            voltage_range: VoltageScale::RANGE1,
            ls: LsConfig {
                rtc: RtcClockSource::DISABLE,
                lsi: false,
                lse: None,
            },
        }
    }
}
