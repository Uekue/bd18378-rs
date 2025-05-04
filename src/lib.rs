//! # BD18378
//! A platform-agnostic Rust driver for the ROHM BD18378 LED Driver IC.
//! This driver is designed to be used with embedded systems and is compatible
//! with the `embedded-hal` crate.

#![no_std]

use embedded_hal::spi::SpiDevice;
use crate::registers::WriteRegister;

pub mod registers;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
/// The `OperationResult` type represents the result of an operation on the BD18378 LED Driver IC.
pub type OperationResult = Result<(), ()>;


/// The `Bd18378` struct represents the ROHM BD18378 LED Driver IC.
pub struct Bd18378<'a, SPI: SpiDevice> {
    spi: &'a mut SPI,
    is_initialized: bool,
    channel_enable: [bool; 12],
}

    }
}
