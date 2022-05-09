mod error;
mod traits;

#[macro_use]
pub mod common;

#[derive(Debug, Default)]
pub struct Transport;

#[cfg(feature = "hal")]
mod hal;

#[cfg(feature = "rppal")]
mod rppal;

#[cfg(feature = "rp2040")]
mod rp2040;

pub use {
    error::{Error, Result},
    traits::{ChipSelect, ClockSpeed, SpiDev},
};
