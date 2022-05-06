mod error;
mod traits;

/// [`ByteTransport`] for SPI devices which manage chip selection.
pub mod auto;

/// [`ByteTransport`] for SPI devices which require a chip select controller.
pub mod cs;

pub use {
    error::Error,
    traits::{ByteTransport, ChipSelect, ClockSpeed, Result},
};
