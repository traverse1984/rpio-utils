/// Indicates an SPI error. The [`Transport`](super::traits::Transport)
/// in use determines which errors are possible. Of the built-in transports:
///
/// - Any transport can encounter [Transfer](Error::Transfer).
/// - Implementors of [`ChipSelect`](super::traits::ChipSelect) can encounter
///   [ChipSelect](Error::ChipSelect) and [ChipDeselect](Error::ChipDeselect).
/// - Implementors of [`ClockSpeed`](super::traits::ClockSpeed) can encounter
///   [ClockSpeed](Error::ClockSpeed).
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
