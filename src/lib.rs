#![warn(clippy::all)]
#![no_std]
#![cfg(feature = "std")]
extern crate std;

/// Module for creating a [`ByteTransport`] from various SPI configurations.
mod transport;
pub use transport::*;

#[cfg(feature = "dev")]
/// Utilities for creating mocks and intercepts.
pub mod dev;

/// Traits and Enums from [`embedded_hal`] which are used within this library.
pub mod hal {
    pub use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin, spi::Polarity};
}
