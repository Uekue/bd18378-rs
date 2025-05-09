//! # BD18378
//! A platform-agnostic Rust driver for the ROHM BD18378 LED Driver IC.
//! This driver is designed to be used with embedded systems and is compatible
//! with the `embedded-hal` crate.

#![no_std]

use embedded_hal::spi::SpiDevice;
use crate::registers::WriteRegister;

pub mod registers;

/// The `Error` enum represents various error types that can occur during
/// communication with the BD18378 LED Driver IC.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {

    /// Indicates a bus error during SPI communication coming from the used SPI device.
    BusError,

    /// Indicates a communication error during SPI communication due to an unexpected response.
    CommunicationError,

    /// Indicates that the device was not in an initialized state after completing the initialization sequence.
    InitFailed,
    
    /// Indicates that the device was not in an initialized state when trying to perform an operation.
    NotInitialized,
}

/// The `OperationResult` type represents the result of an operation on the BD18378 LED Driver IC.
pub type OperationResult = Result<(), Error>;


/// The `Bd18378` struct represents the ROHM BD18378 LED Driver IC.
pub struct Bd18378<'a, SPI: SpiDevice> {
    spi: &'a mut SPI,
    is_initialized: bool,
    channel_enable: [bool; 12],
}

impl<'a, SPI: SpiDevice> Bd18378<'a, SPI> {

    /// Creates a new instance of the `Bd18378` struct. It takes a mutable reference
    /// to a SPI device as an argument.
    pub fn new(spi: &'a mut SPI) -> Self {
        Bd18378 {
            spi,
            is_initialized: false,
            channel_enable: [false; 12],
        }
    }

    /// Initializes the BD18378 LED Driver IC by writing a sequence of values to its registers.
    /// The sequence is documented in the datasheet of the IC.
    /// Returns an `OperationResult` indicating success or failure of the initialization sequence.
    pub fn init(&mut self) -> OperationResult {
        let mut old_data = [0x00u8, 0x00u8];
        let seq = Self::get_init_sequence();
        let mut first = true;
        for (reg, value) in seq.iter() {
            let data = self.write_register(*reg, *value)?;
            // Validate the SPI transfer response by comparing it with the previous transaction's data.
            // This ensures the integrity of the communication sequence and guards against unexpected
            // responses from the device, which could indicate a communication error.
            if !first && data != old_data {
                return Err(Error::CommunicationError);
            }
            old_data = [*reg as u8, *value];
            first = false;
        }

        self.reset_status_register()?;
        self.is_initialized = true;
        Ok(())
    }

    /// Returns whether the BD18378 LED Driver IC is initialized.
    ///
    /// *Note: This is not a live view of the IC state, but rather a flag
    /// indicating whether the initialization sequence has been successfully executed.
    /// This behavior might change in the future.*
    pub fn is_initialized(&self) -> bool { self.is_initialized }

    
    /// Update all LED channels based on their enabled state.
    pub fn update_all_channels(&mut self) -> OperationResult {

        if !self.is_initialized() {
            return Err(Error::NotInitialized);
        }

        // first 6 channels
        let mut value = 0u8;
        for ch in 0..6 {
            if self.channel_enable[ch] {
                value |= 1 << ch;
            }
        }
        self.write_register(WriteRegister::ChannelEnable00To05, value)?;

        // last 6 channels
        let mut value = 0u8;
        for ch in 6..12 {
            if self.channel_enable[ch] {
                value |= 1 << ch - 6;
            }
        }
        self.write_register(WriteRegister::ChannelEnable06To11, value)?;

        Ok(())
    }

    /// Writes a value to a specified register of the BD18378 LED Driver IC.
    fn write_register(&mut self, register: WriteRegister, value: u8) -> Result<[u8; 2], Error> {
        let mut data = [register as u8, value];
        let result = self.spi.transfer_in_place(&mut data);
        if result.is_ok() {
            Ok(data)
        } else {
            Err(Error::BusError)
        }
    }

    /// Resets the status register of the BD18378 LED Driver IC.
    fn reset_status_register(&mut self) -> OperationResult {
        let _ = self.write_register(WriteRegister::StatusReset, 0b0011_1111u8)?;
        Ok(())
    }

    /// A placeholder function for locking the BD18378 LED Driver IC's registers.
    ///
    /// This function is currently a no-op but is reserved for future functionality
    /// where register locking might be required to prevent unintended modifications.
    /// 
    /// *Note: This function is private and not used in the current implementation.*
    fn _lock_register(&mut self) -> Result<(), ()> { Ok(()) }

    /// Returns the initialization sequence for the BD18378 LED Driver IC.
    const fn get_init_sequence() -> [(WriteRegister, u8); 15] {
        [
            (WriteRegister::SoftwareReset, 0b1010_0001u8),
            (WriteRegister::SoftwareReset, 0b1010_0001u8),
            (WriteRegister::ReservedB5, 0b1001_1110u8),
            (WriteRegister::ReservedB6, 0b0000_0000u8),
            (WriteRegister::ReservedB5, 0b1001_1110u8),
            (WriteRegister::ReservedB7, 0b0000_0000u8),
            (WriteRegister::ReservedB5, 0b1001_1110u8),
            (WriteRegister::ReservedB8, 0b0000_0000u8),
            (WriteRegister::ReservedB5, 0b1001_1110u8),
            (WriteRegister::ReservedB9, 0b0000_0000u8),
            (WriteRegister::Reserved79, 0b1101_0110u8),
            (WriteRegister::Reserved7A, 0b0000_0000u8),
            (WriteRegister::Reserved79, 0b1101_0110u8),
            (WriteRegister::Reserved7B, 0b0000_0000u8),
            (WriteRegister::SoftwareReset, 0b1010_0001u8),
        ]
    }
}
