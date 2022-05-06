use crate::transport::{auto, cs};
use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin, spi::Polarity};

/// Create an SPI transport.
pub struct Transport<SPI: Transfer<u8>> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> Transport<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub fn with_chip_select<CS: OutputPin>(self, cs: CS) -> ChipSelectTransport<SPI, CS> {
        ChipSelectTransport {
            spi: self.spi,
            polarity: Polarity::IdleHigh,
            cs,
        }
    }

    pub fn init(self) -> auto::Transport<SPI> {
        auto::Transport::new(self.spi)
    }
}

/// Builder for SPI configurations using a chip-select pin.
pub struct ChipSelectTransport<SPI: Transfer<u8>, CS: OutputPin> {
    spi: SPI,
    cs: CS,
    polarity: Polarity,
}

impl<SPI: Transfer<u8>, CS: OutputPin> ChipSelectTransport<SPI, CS> {
    pub fn with_polarity(mut self, polarity: Polarity) -> Self {
        self.polarity = polarity;
        self
    }

    pub fn init(self) -> cs::Transport<SPI, CS> {
        cs::Transport::new(self.spi, self.cs, self.polarity)
    }
}
