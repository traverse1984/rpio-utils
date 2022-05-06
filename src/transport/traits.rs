use super::error::Error;
use embedded_hal::blocking::spi::Transfer;

pub type Result<T = ()> = core::result::Result<T, Error>;

pub trait ByteTransport: Transfer<u8> {}

pub trait AutoSelect: ByteTransport {}

pub trait ChipSelect: ByteTransport {
    fn select(&mut self) -> Result;
    fn deselect(&mut self) -> Result;
    fn transfer_or_deselect<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]>;
}

pub trait ClockSpeed: ByteTransport {
    fn set_clock_speed(&mut self, speed: u32) -> Result;
}
