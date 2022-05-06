use super::error::Error;

pub type Result<T = ()> = core::result::Result<T, Error>;

pub trait ChipSelect {
    fn select(&mut self) -> Result;
    fn deselect(&mut self) -> Result;
    fn transfer_or_deselect<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]>;
}

pub trait ClockSpeed {
    fn set_clock_speed(&mut self, speed: u32) -> Result;
}
