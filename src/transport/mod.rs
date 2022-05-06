mod error;
mod traits;

pub use error::Error;
pub use traits::{AutoSelect, ChipSelect, ClockSpeed, Result};

pub mod auto;
pub mod cs;
