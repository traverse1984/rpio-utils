#![warn(clippy::all)]
#![no_std]
#![cfg(feature = "std")]
extern crate std;

mod transport;
pub use transport::*;

#[cfg(feature = "dev")]
pub mod dev;

#[cfg(feature = "hal")]
pub mod hal {
    pub use embedded_hal::blocking::spi::Transfer;
    pub use embedded_hal::digital::v2::OutputPin;
}
