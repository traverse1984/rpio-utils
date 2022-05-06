use super::{
    error::SpiError,
    intercept::{Spi, SpiOpts},
};
use embedded_hal::blocking::spi::Transfer;
use std::{
    borrow::ToOwned, boxed::Box, cell::RefCell, fmt, rc::Rc, string::String, thread,
    time::Duration, vec::Vec,
};

pub trait ByteGenerator: Fn(&[u8]) -> Vec<u8> {}

pub type BoxedGenerator = Box<dyn ByteGenerator>;
pub type Generator = fn(&[u8]) -> Vec<u8>;

impl ByteGenerator for fn(&[u8]) -> Vec<u8> {}

impl fmt::Debug for dyn ByteGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function (Byte Generator)")
    }
}

#[derive(Debug)]
pub struct MockSpi {
    dev: Rc<RefCell<MockSpiDevice>>,
}

impl MockSpi {
    pub fn new(dev: Rc<RefCell<MockSpiDevice>>) -> Self {
        Self { dev }
    }
}

impl Transfer<u8> for MockSpi {
    type Error = SpiError;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.dev.borrow_mut().transfer(words)
    }
}

#[derive(Debug)]
pub struct MockSpiDevice {
    opts: Rc<RefCell<SpiOpts>>,
    generator: Option<BoxedGenerator>,
    byte_delay: Option<Duration>,
    error: Option<SpiError>,
    error_after_bytes: usize,
}

impl MockSpiDevice {
    pub fn new(opts: Rc<RefCell<SpiOpts>>) -> Self {
        Self {
            opts,
            generator: None,
            byte_delay: None,
            error: None,
            error_after_bytes: 0,
        }
    }
}

impl Transfer<u8> for MockSpiDevice {
    type Error = SpiError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        let rx = match &mut self.generator {
            Some(generator) => generator(words),
            None => Vec::new(),
        };

        for (index, word) in words.iter_mut().enumerate() {
            if self.error.is_some() && self.error_after_bytes == index {
                return Err(self.error.take().unwrap());
            }

            *word = *rx.get(index).unwrap_or(&0x00);

            if self.byte_delay.is_some() {
                thread::sleep(self.byte_delay.unwrap());
            }
        }

        Ok(words)
    }
}

#[derive(Debug)]
pub struct SpiControl {
    spi: Rc<RefCell<MockSpiDevice>>,
}

impl SpiControl {
    pub fn new(spi: Rc<RefCell<MockSpiDevice>>) -> Self {
        Self { spi }
    }

    pub fn set_log(&mut self, log: bool) {
        self.spi.borrow_mut().opts.borrow_mut().log = log;
    }

    pub fn set_log_bytes(&mut self, bytes: bool) {
        self.spi.borrow_mut().opts.borrow_mut().bytes = bytes;
    }

    pub fn set_generator(&mut self, generator: Generator) {
        self.spi.borrow_mut().generator = Some(Box::new(generator));
    }

    pub fn set_boxed_generator(&mut self, generator: BoxedGenerator) {
        self.spi.borrow_mut().generator = Some(generator);
    }

    pub fn clear_generator(&mut self) {
        self.spi.borrow_mut().generator = None;
    }

    pub fn set_byte_delay(&mut self, duration: Duration) {
        self.spi.borrow_mut().byte_delay = Some(duration);
    }

    pub fn clear_byte_delay(&mut self) {
        self.spi.borrow_mut().byte_delay = None;
    }

    pub fn set_error(&mut self, error: SpiError) {
        self.spi.borrow_mut().error = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.spi.borrow_mut().error = None;
    }
}

builder!(mock => MockBuilder<SpiOpts> + Debug {
    byte_delay: Option<Duration> = None,
    generator: Option<BoxedGenerator> = None,
});

impl MockBuilder {
    pub fn with_byte_log(mut self) -> Self {
        self.opts.log = true;
        self.opts.bytes = true;
        self
    }

    pub fn with_generator(mut self, generator: Generator) -> Self {
        self.generator = Some(Box::new(generator));
        self
    }

    pub fn with_boxed_generator(mut self, generator: BoxedGenerator) -> Self {
        self.generator = Some(generator);
        self
    }

    pub fn with_byte_delay(mut self, byte_delay: Duration) -> Self {
        self.byte_delay.replace(byte_delay);
        self
    }

    pub fn init(self) -> (Spi<MockSpi>, SpiControl) {
        let opts = Rc::new(RefCell::new(self.opts));
        let dev = Rc::new(RefCell::new(MockSpiDevice::new(opts.clone())));
        let mut control = SpiControl::new(dev.clone());
        let pin = Spi::new(self.name, MockSpi::new(dev), opts);

        if let Some(delay) = self.byte_delay {
            control.set_byte_delay(delay);
        }

        if let Some(generator) = self.generator {
            control.set_boxed_generator(generator);
        }

        (pin, control)
    }
}
