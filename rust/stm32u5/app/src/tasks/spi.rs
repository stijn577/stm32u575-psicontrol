use embassy_executor::task;
use setup::typedefs::Spi2;

#[task]pub async fn spi_comm(mut spi: Spi2){
    let mut buffer = [0u8; 1024];

    loop {
        let _ = spi.read(&mut buffer).await;
        let _ = spi.write(&buffer).await;
    }
}
