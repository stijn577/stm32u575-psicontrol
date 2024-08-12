use core::str;

use defmt::{info, warn};
use embassy_executor::task;
use embassy_time::Timer;
use setup::typedefs::Spi2;

#[task]
pub async fn spi_comm(mut spi: Spi2){
    let buffer = "Hello from SPI!".as_bytes();

    loop {
        // this is just to demonstrate, normally you would have a cs pin that you set low/high manually 
        // you could also write a struct that manages all CS pins, or you could compose a trait onto the struct
        // allowing you to pass a &mut Output into it, which then does the CS set low and high operation more safely
        // now you could for example forget to set the CS back to the passive level, it can become difficult
        // have an overview when using more than a handful of peripherals
        match spi.write(&buffer).await {
            Ok(_) => {
                if let Ok(output) = str::from_utf8(&buffer) { 
                info!("Spi write succesful! wrote: {:?}", output)
                }
            },
            Err(_) => warn!("Failed to write to spi"),
        }
        Timer::after_secs(7).await;
    }
}
