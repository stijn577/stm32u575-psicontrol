use embassy_stm32::{
    peripherals::{GPDMA1_CH10, GPDMA1_CH11, USART1},
    usart::Uart,
};

pub type Uart1 = Uart<'static, USART1, GPDMA1_CH10, GPDMA1_CH11>;
