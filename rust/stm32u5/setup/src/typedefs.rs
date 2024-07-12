use embassy_stm32::{exti::ExtiInput, gpio::Output, mode::Async, peripherals::TIM4, spi::Spi, timer::simple_pwm::SimplePwm, usart::Uart};

pub type Led = Output<'static>;
pub type Btn = ExtiInput<'static>;
pub type Uart1 = Uart<'static, Async>;
pub type Pwm = SimplePwm<'static, TIM4>;
pub type Spi2 = Spi<'static, Async>;
