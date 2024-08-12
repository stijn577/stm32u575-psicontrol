use crate::messages::{Message, MessageError, MessageTransceiver};
use embassy_stm32::{mode::Async, usart::Uart};

impl MessageTransceiver for Uart<'_, Async> {
    async fn send_message(&mut self, msg: &Message) -> Result<(), MessageError> {
        let mut buf = [0u8; 4096];
        ciborium::into_writer(msg, buf.as_mut_slice()).unwrap();

        let null_term = buf.iter().enumerate().find(|(_, &c)| c == 0x00);
        if let Some(null_term) = null_term {
            self.write(&buf[0..null_term.0]).await.map_err(MessageError::Uart)?;
        } else {
            self.write(&buf).await.map_err(MessageError::Uart)?;
        }

        #[cfg(feature = "defmt")]
        defmt::debug!("Sucessfully wrote message: {:?}", msg);

        Ok(())
    }

    async fn send_message_with_buffer<const N: usize>(&mut self, msg: &Message) -> Result<(), MessageError> {
        let mut buf = [0u8; N];
        ciborium::into_writer(msg, buf.as_mut_slice()).map_err(|_| MessageError::SerializationError)?;

        #[cfg(feature = "defmt")]
        defmt::debug!("Sucessfully wrote message: {:#?}", msg);

        Ok(())
    }

    async fn receive_message(&mut self) -> Result<Message, MessageError> {
        let mut buf = [0u8; 4096];
        let _len = self.read_until_idle(&mut buf).await.map_err(MessageError::Uart)?;
        let msg = ciborium::from_reader(buf.as_slice()).map_err(|_| MessageError::DeserializationError);

        #[cfg(feature = "defmt")]
        defmt::debug!("Received: {:?}", msg);

        msg
    }

    async fn receive_message_with_buffer<const N: usize>(&mut self) -> Result<Message, MessageError> {
        let mut buf = [0u8; N];
        let mut scratch_buf = [0u8; N];

        let _len = self.read_until_idle(&mut buf).await.map_err(MessageError::Uart)?;
        let msg = ciborium::from_reader_with_buffer(buf.as_slice(), scratch_buf.as_mut_slice()).map_err(|_| MessageError::DeserializationError);

        #[cfg(feature = "defmt")]
        defmt::debug!("Received: {:?}", msg);

        msg
    }
}
