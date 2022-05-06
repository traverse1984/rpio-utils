pub mod intercept;
pub mod mock;

pub use {
    intercept::intercept,
    mock::{mock, SpiError},
};
