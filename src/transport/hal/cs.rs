use super::super::{Error, Result};
use crate::{ChipSelect, OutputPin, Polarity, SpiDev, Transfer};

/// Construct a [`Transport`] from an SPI device, chip select pin
/// and [`Polarity`].
pub struct Transport<SPI: Transfer<u8>, CS: OutputPin> {
    spi: SPI,
    cs: CS,
    polarity: Polarity,
}

impl<SPI: Transfer<u8>, CS: OutputPin> Transport<SPI, CS> {
    pub fn new(spi: SPI, cs: CS, polarity: Polarity) -> Self {
        let mut transport = Self { spi, cs, polarity };

        transport.deselect().ok();
        transport
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> Transfer<u8> for Transport<SPI, CS> {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.select()
            .and_then(|_| self.raw_transfer_or_deselect(words))
            .and_then(|res| self.deselect().and(Ok(res)))
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> SpiDev for Transport<SPI, CS> {
    fn is_chip_select(&self) -> bool {
        true
    }

    fn select(&mut self) -> Result {
        match self.polarity {
            Polarity::IdleHigh => self.cs.set_low(),
            Polarity::IdleLow => self.cs.set_high(),
        }
        .or(Err(Error::ChipSelect))
    }

    fn deselect(&mut self) -> Result {
        match self.polarity {
            Polarity::IdleHigh => self.cs.set_high(),
            Polarity::IdleLow => self.cs.set_low(),
        }
        .or(Err(Error::ChipDeselect))
    }

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> ChipSelect for Transport<SPI, CS> {}
