use core::str::FromStr;
use embassy_executor::task;
use embassy_time::Timer;
use functions::{
    messages::{Message, MessageTransceiver},
    password::{PWBuff, Password, Unencrypted},
};
use setup::typedefs::Uart1;

#[task]
pub async fn uart_comm(mut usart1: Uart1) -> ! {
    loop {
        let msg = Message::HelloWorld;
        usart1.send_message(&msg).await.unwrap();
        Timer::after_millis(2000).await;

        let pw = Password::<Unencrypted>::new(PWBuff::from_str("test").unwrap());
        let msg = Message::Password(pw.encrypt());
        usart1.send_message(&msg).await.unwrap();
        Timer::after_millis(2000).await;
    }
}
