use super::{auto, cs};
use crate::{OutputPin, Polarity, Transfer, Transport};

impl Transport {
    /// Construct a transport from any [`Transfer<u8>`](Transfer).
    pub fn hal<SPI: Transfer<u8>>(spi: SPI) -> HalBuilder<SPI> {
        HalBuilder { spi }
    }
}

pub struct HalBuilder<SPI: Transfer<u8>> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> HalBuilder<SPI> {
    /// Use the provided chip select pin.
    pub fn with_cs<CS: OutputPin>(self, cs: CS) -> HalChipSelectBuilder<SPI, CS> {
        HalChipSelectBuilder {
            spi: self.spi,
            polarity: Polarity::IdleHigh,
            cs,
        }
    }

    /// Initialize the transport.
    ///
    /// Chip select must be handled by the provided SPI device.
    pub fn init(self) -> auto::Transport<SPI> {
        auto::Transport::new(self.spi)
    }
}

pub struct HalChipSelectBuilder<SPI: Transfer<u8>, CS: OutputPin> {
    spi: SPI,
    cs: CS,
    polarity: Polarity,
}

impl<SPI: Transfer<u8>, CS: OutputPin> HalChipSelectBuilder<SPI, CS> {
    /// Use the provided polarity. Defaults to [IdleHigh](Polarity::IdleHigh).
    pub fn with_polarity(mut self, polarity: Polarity) -> Self {
        self.polarity = polarity;
        self
    }

    /// Initialize the transport.
    pub fn init(self) -> cs::Transport<SPI, CS> {
        cs::Transport::new(self.spi, self.cs, self.polarity)
    }
}
