use super::super::{Error, Result};
use crate::{ChipSelect, ClockSpeed, Polarity, SpiDev, Transfer};
use _rppal::{gpio::OutputPin as RpPin, spi::Spi};

pub struct Transport {
    spi: Spi,
    cs: RpPin,
    polarity: Polarity,
}

impl Transport {
    pub fn new(spi: Spi, cs: RpPin, polarity: Polarity) -> Self {
        let mut transport = Self { spi, cs, polarity };

        transport.deselect().ok();
        transport
    }
}

impl SpiDev for Transport {
    fn is_chip_select(&self) -> bool {
        true
    }

    fn select(&mut self) -> Result {
        match self.polarity {
            Polarity::IdleHigh => self.cs.set_low(),
            Polarity::IdleLow => self.cs.set_high(),
        };

        Ok(())
    }

    fn deselect(&mut self) -> Result {
        match self.polarity {
            Polarity::IdleHigh => self.cs.set_high(),
            Polarity::IdleLow => self.cs.set_low(),
        };

        Ok(())
    }

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        <Spi as Transfer<u8>>::transfer(&mut self.spi, words).or(Err(Error::Transfer))
    }

    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}

impl Transfer<u8> for Transport {
    impl_cs_transfer_common!();
}

impl ChipSelect for Transport {}
impl ClockSpeed for Transport {}
