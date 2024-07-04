use defmt::{info, warn};
use embassy_executor::task;
use setup::typedefs::Uart1;

#[task]
pub async fn uart_rx(mut usart1: Uart1) {
    let mut buffer = [0u8; 1024];
    loop {
        match usart1.read(&mut buffer).await {
            Ok(_) => info!("Read from usart1: {:?}", buffer),
            Err(e) => warn!("Reading from usart1 failed: {:?}", e),
        }
        match usart1.write(&buffer).await {
            Ok(_) => info!("Written to usart1: {:?}", buffer),
            Err(e) => warn!("Writing to usart1 failed: {:?}", e),
        }
    }
}
