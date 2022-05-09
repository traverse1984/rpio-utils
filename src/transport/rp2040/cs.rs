use super::super::{Error, Result};
use crate::{ChipSelect, ClockSpeed, OutputPin, Polarity, SpiDev, Transfer};
use embedded_time::rate::{Extensions, Hertz};
use rp2040_hal::{
    gpio::{Pin, PinId, PushPullOutput},
    spi::{Enabled, Spi, SpiDevice},
};

pub struct Transport<D: SpiDevice, P: PinId> {
    spi: Spi<Enabled, D, 8>,
    peripheral_freq: Hertz<u32>,
    cs: Pin<P, PushPullOutput>,
    polarity: Polarity,
}

impl<D: SpiDevice, P: PinId> Transport<D, P> {
    pub fn new(
        spi: Spi<Enabled, D, 8>,
        peripheral_freq: Hertz<u32>,
        cs: Pin<P, PushPullOutput>,
        polarity: Polarity,
    ) -> Self {
        let mut transport = Self {
            spi,
            peripheral_freq,
            cs,
            polarity,
        };

        transport.deselect().ok();
        transport
    }
}

impl<D: SpiDevice, P: PinId> SpiDev for Transport<D, P> {
    impl_cs_common!();

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }

    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_baudrate(self.peripheral_freq, speed.Hz());
        Ok(())
    }
}

impl<D: SpiDevice, P: PinId> Transfer<u8> for Transport<D, P> {
    impl_cs_transfer_common!();
}

impl<D: SpiDevice, P: PinId> ChipSelect for Transport<D, P> {}
impl<D: SpiDevice, P: PinId> ClockSpeed for Transport<D, P> {}
