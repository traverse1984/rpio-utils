use super::{
    error::Error,
    traits::{AutoSelect, ClockSpeed, Result},
};
use embedded_hal::blocking::spi::Transfer;

#[cfg(feature = "rppal")]
use _rppal::spi::Spi;

pub struct Transport<SPI> {
    spi: SPI,
}

#[cfg(feature = "hal")]
impl<SPI: Transfer<u8>> Transport<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

#[cfg(feature = "hal")]
impl<SPI: Transfer<u8>> AutoSelect for Transport<SPI> {}

#[cfg(feature = "hal")]
impl<SPI: Transfer<u8>> Transfer<u8> for Transport<SPI> {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }
}

#[cfg(feature = "rppal")]
impl ClockSpeed for Transport<Spi> {
    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}
