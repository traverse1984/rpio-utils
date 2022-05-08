use super::{auto, cs};
use crate::{OutputPin, Polarity, Transfer, Transport};

impl Transport {
    pub fn hal<SPI: Transfer<u8>>(spi: SPI) -> HalBuilder<SPI> {
        HalBuilder { spi }
    }
}

pub struct HalBuilder<SPI: Transfer<u8>> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> HalBuilder<SPI> {
    pub fn with_cs<CS: OutputPin>(self, cs: CS) -> HalChipSelectBuilder<SPI, CS> {
        HalChipSelectBuilder {
            spi: self.spi,
            polarity: Polarity::IdleHigh,
            cs,
        }
    }

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
    pub fn with_polarity(mut self, polarity: Polarity) -> Self {
        self.polarity = polarity;
        self
    }

    pub fn init(self) -> cs::Transport<SPI, CS> {
        cs::Transport::new(self.spi, self.cs, self.polarity)
    }
}
