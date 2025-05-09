extern crate alloc;

use alloc::vec;
use bd18378::Bd18378;
use bd18378::registers::WriteRegister;
use embedded_hal_mock::eh1::spi::{Mock, Transaction};

mod common;

#[test]
fn chip_init_success() {
    let expectations = common::get_init_sequence_spi_expectations();
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    let result = bd18378.init();

    assert!(result.is_ok());
    assert!(bd18378.is_initialized());

    spi.done();
}

#[test]
fn chip_init_fail_no_answer_pull_down() {
    let expectations = [
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
            vec![0x00, 0x00],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
            vec![0x00, 0x00],
        ),
        Transaction::transaction_end(),
    ];
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    let result = bd18378.init();

    assert!(result.is_err());
    assert!(!bd18378.is_initialized());

    spi.done();
}

#[test]
fn chip_init_fail_no_answer_pull_up() {
    let expectations = [
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
            vec![0xFF, 0xFF],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
            vec![0xFF, 0xFF],
        ),
        Transaction::transaction_end(),
    ];
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    let result = bd18378.init();

    assert!(result.is_err());
    assert!(!bd18378.is_initialized());

    spi.done();
}
