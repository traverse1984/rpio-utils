use super::{auto, cs};
use crate::{Polarity, Transport};
use embedded_time::rate::Hertz;
use rp2040_hal::{
    gpio::{Pin, PinId, PushPullOutput},
    spi::{Enabled, Spi, SpiDevice},
};

impl Transport {
    /// Construct a transport from an [`rp2040::spi::Spi`](Spi).
    pub fn rp2040<D: SpiDevice>(
        spi: Spi<Enabled, D, 8>,
        peripheral_freq: impl Into<Hertz>,
    ) -> Rp2040Builder<D> {
        Rp2040Builder {
            spi,
            peripheral_freq: peripheral_freq.into(),
        }
    }
}

pub struct Rp2040Builder<D: SpiDevice> {
    spi: Spi<Enabled, D, 8>,
    peripheral_freq: Hertz,
}

impl<D: SpiDevice> Rp2040Builder<D> {
    /// Use the provided [`rp2040::gpio::Pin`](Pin) for chip select. It must
    /// be configured as a [`PushPullOutput`].
    pub fn with_cs<P: PinId>(self, cs: Pin<P, PushPullOutput>) -> Rp2040ChipSelectBuilder<D, P> {
        Rp2040ChipSelectBuilder {
            spi: self.spi,
            peripheral_freq: self.peripheral_freq,
            polarity: Polarity::IdleHigh,
            cs,
        }
    }

    /// Initialize the transport.
    pub fn init(self) -> auto::Transport<D> {
        auto::Transport::new(self.spi, self.peripheral_freq)
    }
}

pub struct Rp2040ChipSelectBuilder<D: SpiDevice, P: PinId> {
    spi: Spi<Enabled, D, 8>,
    peripheral_freq: Hertz,
    cs: Pin<P, PushPullOutput>,
    polarity: Polarity,
}

impl<D: SpiDevice, P: PinId> Rp2040ChipSelectBuilder<D, P> {
    /// Use the provided polarity. Defaults to [IdleHigh](Polarity::IdleHigh).
    pub fn with_polarity(mut self, polarity: Polarity) -> Self {
        self.polarity = polarity;
        self
    }

    /// Initialize the transport.
    pub fn init(self) -> cs::Transport<D, P> {
        cs::Transport::new(self.spi, self.peripheral_freq, self.cs, self.polarity)
    }
}
