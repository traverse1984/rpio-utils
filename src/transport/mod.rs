mod error;
mod traits;

pub use error::Error;
pub use traits::{ChipSelect, ClockSpeed, Result};

pub mod auto;
pub mod cs;
