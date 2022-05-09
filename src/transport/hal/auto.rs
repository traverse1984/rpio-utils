use super::super::{Error, Result};
use crate::{SpiDev, Transfer};

pub struct Transport<SPI: Transfer<u8>> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> Transport<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

impl<SPI: Transfer<u8>> Transfer<u8> for Transport<SPI> {
    impl_auto_transfer_common!();
}

impl<SPI: Transfer<u8>> SpiDev for Transport<SPI> {}
