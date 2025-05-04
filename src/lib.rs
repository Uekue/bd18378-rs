//! # BD18378
//! A platform-agnostic Rust driver for the ROHM BD18378 LED Driver IC.
//! This driver is designed to be used with embedded systems and is compatible
//! with the `embedded-hal` crate.

#![no_std]

use embedded_hal::spi::SpiDevice;
use crate::registers::WriteRegister;

pub mod registers;

/// The `OperationResult` type represents the result of an operation on the BD18378 LED Driver IC.
pub type OperationResult = Result<(), ()>;


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

    /// Returns whether the BD18378 LED Driver IC is initialized.
    ///
    /// *Note: This is not a live view of the IC state, but rather a flag
    /// indicating whether the initialization sequence has been successfully executed.
    /// This behavior might change in the future.*
    pub fn is_initialized(&self) -> bool { self.is_initialized }


    /// Writes a value to a specified register of the BD18378 LED Driver IC.
    fn write_register(&mut self, register: WriteRegister, value: u8) -> Result<[u8; 2], ()> {
        let mut data = [register as u8, value];
        let result = self.spi.transfer_in_place(&mut data);
        if result.is_ok() {
            Ok(data)
        } else {
            Err(())
        }
    }


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
