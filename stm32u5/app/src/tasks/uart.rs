// use defmt::{info, warn};
use embassy_executor::task;
use setup::typedefs::Uart1;

#[task]
pub async fn uart_rx(mut usart1: Uart1) {
    let mut buffer = [0u8; 1024];
    loop {
        usart1.read(&mut buffer).await; 
        usart1.write(&buffer).await;
    }
}
