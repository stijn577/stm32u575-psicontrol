use defmt::{info, warn};

use embassy_executor::task;
use setup::typedefs::Uart1;

#[task]
pub async fn uart_comm(mut usart1: Uart1) {
    let mut buffer = [0u8; 1024];
    loop {
        match usart1.read(&mut buffer).await {
            Ok(_) => info!("Read from uart succes: {:?}", &buffer),
            Err(_) => warn!("Failed to read from uart"),
        }
        match usart1.write(&buffer).await {
            Ok(_) => info!("Wrote to uart succes: {:?}", &buffer),
            Err(_) => warn!("Failed to write to uart"),
        }
    }
}
