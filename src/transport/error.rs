#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    Transfer,
    ChipSelect,
    ChipDeselect,
    ClockSpeed,
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Transfer => "SPI transfer error",
                Error::ChipSelect => "Select SPI chip error",
                Error::ChipDeselect => "Deselect SPI chip error",
                Error::ClockSpeed => "Set SPI clock speed error",
            }
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
