extern crate alloc;

use alloc::vec;
use bd18378::Bd18378;
use bd18378::registers::WriteRegister;
use embedded_hal_mock::eh1::spi::{Mock, Transaction};

#[test]
fn chip_init_success() {
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
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB6 as u8, 0b0000_0000u8],
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
            vec![WriteRegister::ReservedB6 as u8, 0b0000_0000u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB7 as u8, 0b0000_0000u8],
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
            vec![WriteRegister::ReservedB7 as u8, 0b0000_0000u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB8 as u8, 0b0000_0000u8],
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
            vec![WriteRegister::ReservedB8 as u8, 0b0000_0000u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::ReservedB9 as u8, 0b0000_0000u8],
            vec![WriteRegister::ReservedB5 as u8, 0b1001_1110u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::Reserved79 as u8, 0b1101_0110u8],
            vec![WriteRegister::ReservedB9 as u8, 0b0000_0000u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::Reserved7A as u8, 0b0000_0000u8],
            vec![WriteRegister::Reserved79 as u8, 0b1101_0110u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::Reserved79 as u8, 0b1101_0110u8],
            vec![WriteRegister::Reserved7A as u8, 0b0000_0000u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::Reserved7B as u8, 0b0000_0000u8],
            vec![WriteRegister::Reserved79 as u8, 0b1101_0110u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::SoftwareReset as u8, 0b1010_0001u8],
            vec![WriteRegister::Reserved7B as u8, 0b0000_0000u8],
        ),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![WriteRegister::StatusReset as u8, 0x3Fu8],
            vec![0x00u8, 0x00u8],
        ),
        Transaction::transaction_end(),
    ];
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
    ];
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    let result = bd18378.init();

    assert!(result.is_err());
    assert!(!bd18378.is_initialized());

    spi.done();
}
