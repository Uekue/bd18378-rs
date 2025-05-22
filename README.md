# bd18378-rs
Driver for ROHM BD18378 12CH LED Driver IC

A Rust driver for the ROHM BD18378 12CH LED Driver IC, suitable for embedded applications. 
This crate supports `embedded-hal` traits and is fully compatible with `#![no_std]` environments.

## ✨ Features

- Supports the ROHM BD18378 LED driver IC
- Communication via SPI interface 
- Platform-agnostic via `embedded-hal::spi` traits
- Supports `#![no_std]` environments

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
bd18378 = "0.1"
```

or add it via command line using:

```bash
cargo add bd18378
```

## 🔮 Example

```rust
#![no_std]
use bd18378::Bd18378;
use embedded_hal::spi::SpiDevice;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

fn main() {
    // Initialize the SPI interface
    let spi = ...; // Your SPI implementation here
    let cs = ...; // Chip select pin

    let mut spi_dev = ExclusiveDevice::new_no_delay(&mut spi, &mut cs).unwrap();

    // Create a new instance of the BD18378 driver
    let mut led_driver = Bd18378::new(&mut spi_dev);
    
    // Initialize the driver
    led_driver.init().ok();

    // Set the brightness of channel 0 to maximum
    led_driver.set_channel_calibration(0, 0x3Fu8).ok();

    // Turn on channel 0
    led_driver.enable_channel(0).ok();
    led_driver.update_all_channels().ok();
}
```

## 📖 Documentation

Auto-generated API documentation is available at: https://docs.rs/bd18378

## 📚 License

This project is dual-licensed under either:

- MIT License
- Apache License, Version 2.0

You may choose the license that best fits your project.
