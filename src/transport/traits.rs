use super::error::Error;
use embedded_hal::blocking::spi::Transfer;

/// Result where the Err is an SPI [`Error`].
pub type Result<T = ()> = core::result::Result<T, Error>;

/// Indicates that the implementation of [`Transfer<u8>`](Transfer) for this
/// struct:
///
/// - Selects the chip at the start of transfer.
/// - Deselects the chip at the end of successful transfer.
/// - Uses the [`Error`] type.
pub trait ByteTransport: Transfer<u8, Error = Error> {}

/// Indicates that chip selection is controlled by a user-defined output pin.
pub trait ChipSelect: ByteTransport {
    /// Select the chip.
    ///
    /// This typically drives the pin low, but in some configurations could
    /// drive the pin high.
    fn select(&mut self) -> Result;

    /// Deselect the chip.
    ///
    /// This typically drives the pin high, but in some configurations could
    /// drive the pin low.
    fn deselect(&mut self) -> Result;

    /// Exchange bytes with the chip without selecting or deslecting it.
    fn exchange_bytes<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]>;

    /// Exchange bytes with the chip without selecting it. Deselect only if an
    /// error occurs during the transfer.
    fn exchange_bytes_or_deselect<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.exchange_bytes(words)
            .map_err(|err| self.deselect().map_or(Error::ChipDeselect, |_| err))
    }
}

/// Indicates that the SPI clock speed can be set during operation.
pub trait ClockSpeed: ByteTransport {
    /// Set the SPI clock speed.
    fn set_clock_speed(&mut self, speed: u32) -> Result;
}
