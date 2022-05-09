use super::super::{Error, Result};
use crate::{ClockSpeed, SpiDev, Transfer};
use _rppal::spi::Spi;

pub struct Transport {
    spi: Spi,
}

impl Transport {
    pub fn new(spi: Spi) -> Self {
        Self { spi }
    }
}

impl Transfer<u8> for Transport {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        <Spi as Transfer<u8>>::transfer(&mut self.spi, words).or(Err(Error::Transfer))
    }
}

impl SpiDev for Transport {
    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}

impl ClockSpeed for Transport {}
