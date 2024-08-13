use serde::{Deserialize, Serialize};

use crate::password::Encrypted;
use crate::password::Password;

pub type Buff = heapless::String<32>;

/// This is an enum that defines different types of messages, they can be used to communicate between 2 devices with a shared type definition.
///
/// # Variants
///
/// *
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub enum Message {HelloWorld,Password(Password<Encrypted>),}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(thiserror_no_std::Error, Debug)]
pub enum MessageError {
    SerializationError,
    DeserializationError,
    Uart(#[from] embassy_stm32::usart::Error),
}

pub trait MessageTransceiver {
    /// Sends a message over the communication channel. Using a default buffer sizem for [`embassy_stm32::usart::Uart`], 4KB was used for the default buffer. Mostly because [`ciborium`]
    /// uses the same buffer size, so it is an obvious choice to make.
    ///
    /// # Parameters
    ///
    /// * `msg`: A reference to the message to be sent.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to `Ok(())` if the message is successfully sent,
    /// or `Err(MessageError)` if an error occurs during the transmission.
    fn send_message(&mut self, msg: &Message) -> impl core::future::Future<Output = Result<(), MessageError>> + Send;

    /// Sends a message over the communication channel using a buffer of a specified size. But can use a custom buffer size instead. Note that the max buffer size is dependent on the communication channel.
    /// [`embassy_stm32::usart::Uart`] in [`embassy_stm32::mode::Async`] mode for example uses DMA, the max buffer allowed there is 65535 (STM32U575ZITxQ), but you probably will never even come close to those values.
    ///
    /// If you do exceed the BUFFER SIZE of your communication channel, consider chunking the data before sending it, keep in mind you will need to reconstruct it on the other end. (you can then just ignore the const N parameter)
    /// If possible chunking might also be better in normal operation (depending on the communication channel), if it is a slow channel it could be locking the CPU for too long, especially if you just use this trait as a wrapper
    /// to blocking code. But this is up to you to implement as needed.
    ///
    /// # Parameters
    ///
    /// * `msg`: A reference to the message to be sent.
    /// * `N`: A constant representing the size of the buffer to be used for sending the message.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to `Ok(())` if the message is successfully sent,
    /// or `Err(MessageError)` if an error occurs during the transmission.
    fn send_message_with_buffer<const N: usize>(&mut self, msg: &Message) -> impl core::future::Future<Output = Result<(), MessageError>> + Send;

    /// Receives a message from the communication channel.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to `Ok(Message)` if a message is successfully received,
    /// or `Err(MessageError)` if an error occurs during the reception.
    fn receive_message(&mut self) -> impl core::future::Future<Output = Result<Message, MessageError>> + Send;

    /// Receives a message from the communication channel using a buffer of a specified size.
    ///
    /// # Parameters
    ///
    /// * `N`: A constant representing the size of the buffer to be used for receiving the message.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to `Ok(Message)` if a message is successfully received,
    /// or `Err(MessageError)` if an error occurs during the reception.
    fn receive_message_with_buffer<const N: usize>(&mut self) -> impl core::future::Future<Output = Result<Message, MessageError>> + Send;
}
