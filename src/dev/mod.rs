//! Intercept SPI devices or create mock devices. To use, enable the `dev`
//! feature (which requires [`std`]).
//!
//! # Examples
//!
//! ```
//! use rpio_utils::{*, dev::*};
//!
//! let real_spi = ...;
//! let real_cs_pin = ...;
//!
//! let spi_int = Intercept::spi("MySPI").init(real_spi);
//! let pin_int = Intercept::pin("MyCS").init(real_cs_pin);
//!
//! // Now logs SPI traffic
//! let spi = Transport::new(spi_int)
//!     .with_chip_select(pin_int)
//!     .init();
//!
//! ```
//!
//! ## Mocks
//!
//! ```
//! use rpio_utils::{*, dev::*};
//!
//! let (spi, spi_control) = Mock::spi("MockSPI")
//!     .with_byte_log()
//!     .with_generator(|tx: &[u8]| tx.to_vec())
//!     .init();
//!
//! let (cs, cs_control) = Mock::pin("MockCS").init();
//!
//! // Emulated device implementing Transfer<u8>
//! let spi = Transport::new(spi)
//!     .with_chip_select(cs)
//!     .init();
//!
//! // Introduce a transfer error after 33 bytes:
//! spi_control
//!     .set_error(Error::Transfer)
//!     .set_error_defer_bytes(33);
//!
//! // Or on the pin:
//! cs_control
//!     .set_error(PinError::SetHigh)
//! ```

#[macro_use]
mod builder;

pub mod output;
pub mod spi;

pub use {
    builder::{Intercept, Mock},
    output::mock::PinError,
};
