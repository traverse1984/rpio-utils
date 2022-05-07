#![no_std]
#![warn(clippy::all)]

//! Create predictable SPI transports, intercept communication and create mock
//! devices. Uses the [`core`] library and makes use of the [`embedded_hal`]
//! traits.
//!
//! # Examples
//!
//! ```
//! use rpio_utils::*;
//!
//! let real_spi = ...;
//! let real_cs_pin = ...;
//!
//! // When the SPI device handles chip select automatically
//! let spi = Transport::new(real_spi).init();
//!
//! // Then the SPI device needs a CS pin provided
//! let spi = Transport::new(real_spi)
//!     .with_chip_select(real_cs_pin)
//!     .init();
//!
//! // Transfer handles the chip select automatically in both cases:
//! let message = [0x01, 0x02, 0x03, 0x04];
//! let res: &[u8] = spi.transfer(&mut message).unwrap();
//! ```
mod transport;
pub use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin, spi::Polarity};
pub use transport::{ByteTransport, ChipSelect, ClockSpeed, Error, Transport};

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "dev")]
pub mod dev;
