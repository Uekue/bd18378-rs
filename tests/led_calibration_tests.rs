use bd18378::Bd18378;
use embedded_hal_mock::eh1::spi::{Mock, Transaction};

mod common;

#[test]
fn led_calibration_no_init() {

    let expectations: [Transaction<u8>; 0] = [];
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    let result = bd18378.set_channel_calibration(0, 0x05u8);

    assert!(result.is_err());
    assert!(!bd18378.is_initialized());
    assert_eq!(result.unwrap_err(), bd18378::Error::NotInitialized);

    spi.done();
}

#[test]
fn led_calibration_invalid_channel() {

    let expectations: [Transaction<u8>; 0] = [];
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    let result = bd18378.set_channel_calibration(12, 0x05u8);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), bd18378::Error::InvalidChannel);

    spi.done();
}

#[test]
fn led_calibration_success() {

    let init_expectations = common::get_init_sequence_spi_expectations();

    let calibration_expectations = [
        Transaction::transaction_start(),
        Transaction::transfer_in_place(
            vec![bd18378::registers::WriteRegister::ChannelCalibration00 as u8, 0x05u8],
            vec![0x00, 0x00],
        ),
        Transaction::transaction_end(),
    ];

    let mut expectations = init_expectations.to_vec();
    expectations.append(&mut calibration_expectations.to_vec());
    let expectations: [_; 51] = expectations.try_into().unwrap();
    let mut spi = Mock::new(&expectations);

    let mut bd18378 = Bd18378::new(&mut spi);
    bd18378.init().unwrap();
    let result = bd18378.set_channel_calibration(0, 0x05u8);
    assert!(result.is_ok());

    spi.done();
}