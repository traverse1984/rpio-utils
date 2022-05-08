use super::super::{Error, Result};
use crate::{SpiDev, Transfer};

/// Construct a [`Transport`] from an SPI device. The device must manage
/// chip selection and deselection.
pub struct Transport<SPI: Transfer<u8>> {
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

impl<SPI: Transfer<u8>> SpiDev for Transport<SPI> {}
