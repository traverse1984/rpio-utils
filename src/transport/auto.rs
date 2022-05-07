use super::{
    error::Error,
    traits::{ByteTransport, ClockSpeed, Result},
};
use embedded_hal::blocking::spi::Transfer;

#[cfg(feature = "rppal")]
use _rppal::spi::Spi;

/// Construct a [`Transport`] from an SPI device. The device must manage
/// chip selection and deselection.
pub struct Transport<SPI> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> Transport<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

impl<SPI: Transfer<u8>> Transfer<u8> for Transport<SPI> {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }
}

impl<SPI: Transfer<u8>> ByteTransport for Transport<SPI> {}

#[cfg(feature = "rppal")]
impl ClockSpeed for Transport<Spi> {
    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}
