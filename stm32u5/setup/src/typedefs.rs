use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Input, Output},
    peripherals::{GPDMA1_CH10, GPDMA1_CH11, PC13, PC7, TIM4, USART1},
    timer::simple_pwm::SimplePwm,
    usart::Uart,
};

pub type Led = Output<'static, PC7>;
pub type Btn = Input<'static, PC13>;
pub type Uart1 = Uart<'static, USART1, GPDMA1_CH10, GPDMA1_CH11>;
pub type Pwm = SimplePwm<'static, TIM4>;
