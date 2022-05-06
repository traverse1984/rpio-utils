mod error;
pub mod intercept;
pub mod mock;

pub use {error::SpiError, intercept::intercept, mock::mock};
