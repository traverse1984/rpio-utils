use super::super::{Error, Result};
use crate::{ClockSpeed, SpiDev, Transfer};
use embedded_time::rate::{Extensions, Hertz};
use rp2040_hal::spi::{Enabled, Spi, SpiDevice};

pub struct Transport<D: SpiDevice> {
    spi: Spi<Enabled, D, 8>,
    peripheral_freq: Hertz<u32>,
}

impl<D: SpiDevice> Transport<D> {
    pub fn new(spi: Spi<Enabled, D, 8>, peripheral_freq: Hertz<u32>) -> Self {
        Self {
            spi,
            peripheral_freq,
        }
    }
}

impl<D: SpiDevice> Transfer<u8> for Transport<D> {
    impl_auto_transfer_common!();
}

impl<D: SpiDevice> SpiDev for Transport<D> {
    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_baudrate(self.peripheral_freq, speed.Hz());
        Ok(())
    }
}

impl<D: SpiDevice> ClockSpeed for Transport<D> {}
