#[macro_use]
mod builder;

pub use builder::{Intercept, Mock};

/// Utilities for creating output pin mocks and intercepts.
pub mod output_pin;

/// Utilities for creating SPI mocks and intercepts.
pub mod spi;
