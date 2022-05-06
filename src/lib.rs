#![warn(clippy::all)]
#![no_std]
#![cfg(feature = "std")]
extern crate std;

/// Module for creating a [`ByteTransport`] from various SPI configurations.
mod transport;
pub use transport::Transport;

#[cfg(feature = "dev")]
/// Utilities for creating mocks and intercepts.
pub mod dev;
pub use dev::{Intercept, Mock};

/// Include all the traits and enums used in this package
pub mod prelude {
    #[cfg(feature = "dev")]
    pub use super::dev::{output_pin::PinError, spi::SpiError};

    pub use super::transport::{ByteTransport, ChipSelect, ClockSpeed};
    pub use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin, spi::Polarity};
}
