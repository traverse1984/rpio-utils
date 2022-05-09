use super::{auto, cs};
use crate::{Polarity, Transport};
use _rppal::{gpio::OutputPin, spi::Spi};

impl Transport {
    /// Construct a transport from an [`rppal::spi::Spi`](Spi).
    pub fn rppal(spi: Spi) -> RppalBuilder {
        RppalBuilder { spi }
    }
}

pub struct RppalBuilder {
    spi: Spi,
}

impl RppalBuilder {
    /// Use the provided [`rppal::gpio::OutputPin`](OutputPin) for chip select.
    pub fn with_cs(self, cs: OutputPin) -> RppalChipSelectBuilder {
        RppalChipSelectBuilder {
            spi: self.spi,
            polarity: Polarity::IdleHigh,
            cs,
        }
    }

    /// Initialize the transport.
    pub fn init(self) -> auto::Transport {
        auto::Transport::new(self.spi)
    }
}

pub struct RppalChipSelectBuilder {
    spi: Spi,
    cs: OutputPin,
    polarity: Polarity,
}

impl RppalChipSelectBuilder {
    /// Use the provided polarity. Defaults to [IdleHigh](Polarity::IdleHigh).
    pub fn with_polarity(mut self, polarity: Polarity) -> Self {
        self.polarity = polarity;
        self
    }

    /// Initialize the transport.
    pub fn init(self) -> cs::Transport {
        cs::Transport::new(self.spi, self.cs, self.polarity)
    }
}
