use super::super::{Error, Result};
use crate::{ChipSelect, OutputPin, Polarity, SpiDev, Transfer};

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
    impl_cs_transfer_common!();
}

impl<SPI: Transfer<u8>, CS: OutputPin> SpiDev for Transport<SPI, CS> {
    impl_cs_common!();

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> ChipSelect for Transport<SPI, CS> {}
