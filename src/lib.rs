//! # BD18378
//! A platform-agnostic Rust driver for the ROHM BD18378 LED Driver IC.
//! This driver is designed to be used with embedded systems and is compatible
//! with the `embedded-hal` crate.

#![no_std]

use embedded_hal::spi::SpiDevice;
use crate::registers::WriteRegister;

pub mod registers;

/// The number of LED channels per register.
const CHANNELS_PER_REGISTER: usize = 6;

/// The total number of LED channels in the BD18378 LED Driver IC.
const CHANNELS_PER_IC: usize = 12;

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

    /// Indicates that the specified channel index is invalid.
    InvalidChannel,
}

/// The `OperationResult` type represents the result of an operation on the BD18378 LED Driver IC.
pub type OperationResult = Result<(), Error>;


/// The `Bd18378` struct represents the ROHM BD18378 LED Driver IC.
pub struct Bd18378<'a, SPI: SpiDevice> {
    spi: &'a mut SPI,
    is_initialized: bool,
    channel_enable: [bool; CHANNELS_PER_IC],
}

impl<'a, SPI: SpiDevice> Bd18378<'a, SPI> {

    /// Creates a new instance of the `Bd18378` struct. It takes a mutable reference
    /// to a SPI device as an argument.
    pub fn new(spi: &'a mut SPI) -> Self {
        Bd18378 {
            spi,
            is_initialized: false,
            channel_enable: [false; CHANNELS_PER_IC],
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

    /// Enable a single LED channel by its index.
    ///
    /// *Note: This function does not update the LED channel state immediately.
    /// You need to call `update_all_channels()` to apply the changes.*
    pub fn enable_channel(&mut self, ch: usize) -> OperationResult {
        if ch >= self.channel_enable.len() {
            return Err(Error::InvalidChannel);
        }

        self.check_initialized()?;

        self.channel_enable[ch] = true;
        Ok(())
    }

    /// Disable a single LED channel by its index.
    ///
    /// *Note: This function does not update the LED channel state immediately.
    /// You need to call `update_all_channels()` to apply the changes.*
    pub fn disable_channel(&mut self, ch: usize) -> OperationResult {
        if ch >= self.channel_enable.len() {
            return Err(Error::InvalidChannel);
        }

        self.check_initialized()?;

        self.channel_enable[ch] = false;
        Ok(())
    }

    /// Update all LED channels based on their enabled state.
    ///
    /// This function maps the enabled state of each LED channel to specific bits
    /// in two 8-bit registers. The BD18378 LED Driver IC has 12 channels, divided
    /// into two groups of 6 channels each:
    /// - Channels 0 to 5 are mapped to the `ChannelEnable00To05` register.
    /// - Channels 6 to 11 are mapped to the `ChannelEnable06To11` register.
    ///
    /// For each group, the enabled state of a channel is represented by a single bit
    /// in the corresponding register:
    /// - Bit 0 corresponds to the first channel in the group.
    /// - Bit 1 corresponds to the second channel, and so on.
    ///
    /// For example:
    /// - If channel 0 is enabled, bit 0 of `ChannelEnable00To05` is set to 1.
    /// - If channel 6 is enabled, bit 0 of `ChannelEnable06To11` is set to 1.
    ///
    /// The function first processes channels 0 to 5, then channels 6 to 11, updating
    /// the corresponding registers with the computed bit values.
    pub fn update_all_channels(&mut self) -> OperationResult {

        self.check_initialized()?;

        // first 6 channels
        let first_group_value = self.compute_channel_group_value(0, CHANNELS_PER_REGISTER, 0);
        self.write_register(WriteRegister::ChannelEnable00To05, first_group_value)?;

        let second_group_value = self.compute_channel_group_value(CHANNELS_PER_REGISTER, CHANNELS_PER_IC, CHANNELS_PER_REGISTER);
        self.write_register(WriteRegister::ChannelEnable06To11, second_group_value)?;

        Ok(())
    }

    /// Helper function to compute the value for a group of channels.
    fn compute_channel_group_value(&self, start: usize, end: usize, offset: usize) -> u8 {
        let mut group_value = 0u8;
        for ch in start..end {
            if self.channel_enable[ch] {
                group_value |= 1 << (ch - offset);
            }
        }
        group_value
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
    
    /// Checks if the BD18378 LED Driver IC is initialized before performing any operation.
    fn check_initialized(&self) -> OperationResult {
        if !self.is_initialized {
            return Err(Error::NotInitialized);
        }
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
