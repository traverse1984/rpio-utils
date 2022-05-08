mod error;
mod traits;

mod hal;

pub struct Transport;

pub use {
    error::{Error, Result},
    traits::{ChipSelect, ClockSpeed, SpiDev},
};
