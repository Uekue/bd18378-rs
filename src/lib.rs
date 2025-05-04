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
    }
}
