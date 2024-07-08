// use defmt::{info, warn};
use embassy_executor::task;
use setup::typedefs::Uart1;

#[task]
pub async fn uart_rx(mut usart1: Uart1) {
    let mut buffer = [0u8; 1024];
    loop {
        let _ = usart1.read(&mut buffer).await; 
        let _ = usart1.write(&buffer).await;
    }
}
