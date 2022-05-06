use super::{
    error::Error,
    traits::{ChipSelect, ClockSpeed, Result},
};

use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin, spi::Polarity};

#[cfg(feature = "rppal")]
use _rppal::spi::Spi;

pub struct Transport<SPI, CS> {
    spi: SPI,
    cs: CS,
    polarity: Polarity,
}

#[cfg(feature = "hal")]
impl<SPI: Transfer<u8>, CS: OutputPin> Transport<SPI, CS> {
    pub fn new(spi: SPI, cs: CS, polarity: Polarity) -> Self {
        let mut transport = Self { spi, cs, polarity };

        transport.deselect().ok();
        transport
    }
}

#[cfg(feature = "hal")]
impl<SPI: Transfer<u8>, CS: OutputPin> ChipSelect for Transport<SPI, CS> {
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

    fn transfer_or_deselect<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).map_err(|_| {
            self.deselect()
                .map_or(Error::ChipDeselect, |_| Error::Transfer)
        })
    }
}

#[cfg(feature = "hal")]
impl<SPI: Transfer<u8>, CS: OutputPin> Transfer<u8> for Transport<SPI, CS> {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.select()
            .and_then(|_| self.transfer_or_deselect(words))
            .and_then(|res| self.deselect().and(Ok(res)))
    }
}

#[cfg(feature = "rppal")]
impl<CS: OutputPin> ClockSpeed for Transport<Spi, CS> {
    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}
