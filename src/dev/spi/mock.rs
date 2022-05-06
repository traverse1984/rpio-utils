use super::intercept::{Spi, SpiOpts};
use embedded_hal::blocking::spi::Transfer;
use std::{
    borrow::ToOwned, boxed::Box, cell::RefCell, fmt, rc::Rc, string::String, thread,
    time::Duration, vec::Vec,
};

/// Indicates that the function can be used to generate bytes for mock SPI.
pub trait ByteGenerator: Fn(&[u8]) -> Vec<u8> {}

/// A boxed byte generator for mock SPI
pub type BoxedGenerator = Box<dyn ByteGenerator>;

/// A byte generator for mock SPI
pub type Generator = fn(&[u8]) -> Vec<u8>;

impl ByteGenerator for fn(&[u8]) -> Vec<u8> {}

impl fmt::Debug for dyn ByteGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function (Byte Generator)")
    }
}

/// Transfer interface for Mock SPI.
#[derive(Debug)]
pub struct MockSpi {
    dev: Rc<RefCell<MockSpiDevice>>,
}

impl MockSpi {
    fn new(dev: Rc<RefCell<MockSpiDevice>>) -> Self {
        Self { dev }
    }
}

impl Transfer<u8> for MockSpi {
    type Error = SpiError;
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.dev.borrow_mut().transfer(words)
    }
}

/// An enum of mock SPI errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    Transfer,
}

/// Holds the underlying state shared by a [MockSpi] and an [SpiControl].
#[derive(Debug)]
struct MockSpiDevice {
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
                self.error_after_bytes = 0;
                return Err(self.error.take().unwrap());
            }

            *word = *rx.get(index).unwrap_or(&0x00);

            if self.byte_delay.is_some() {
                thread::sleep(self.byte_delay.unwrap());
            }
        }

        self.error_after_bytes = self.error_after_bytes.saturating_sub(words.len());

        Ok(words)
    }
}

/// Developer controls for mock SPI.
#[derive(Debug)]
pub struct SpiControl {
    spi: Rc<RefCell<MockSpiDevice>>,
}

impl SpiControl {
    fn new(spi: Rc<RefCell<MockSpiDevice>>) -> Self {
        Self { spi }
    }

    /// Set whether events are printed to stdout.
    pub fn set_log(&mut self, log: bool) {
        self.spi.borrow_mut().opts.borrow_mut().log = log;
    }

    /// Set whether Tx/Rx bytes are printed after transfer.
    pub fn set_log_bytes(&mut self, bytes: bool) {
        self.spi.borrow_mut().opts.borrow_mut().bytes = bytes;
    }

    /// Use this function to provide Rx bytes.
    pub fn set_generator(&mut self, generator: Generator) {
        self.spi.borrow_mut().generator = Some(Box::new(generator));
    }

    /// Use this boxed function to provide Rx bytes.
    pub fn set_boxed_generator(&mut self, generator: BoxedGenerator) {
        self.spi.borrow_mut().generator = Some(generator);
    }

    /// Clear the Rx byte generator function (if set).
    pub fn clear_generator(&mut self) {
        self.spi.borrow_mut().generator = None;
    }

    /// Set the per-byte time delay for transfers.
    pub fn set_byte_delay(&mut self, duration: Duration) {
        self.spi.borrow_mut().byte_delay = Some(duration);
    }

    /// Remove the per-byte time delay (if set).
    pub fn clear_byte_delay(&mut self) {
        self.spi.borrow_mut().byte_delay = None;
    }

    /// Set a mock error. The next time this error could occur - it does.
    /// The error may deferred with `set_error_defer_bytes`.
    pub fn set_error(&mut self, error: SpiError) {
        self.spi.borrow_mut().error = Some(error);
    }

    /// Set a byte counter to defer mock errors. Until that many bytes have
    /// been transferred, mock errors will not occur.
    pub fn set_error_defer_bytes(&mut self, defer: usize) {
        self.spi.borrow_mut().error_after_bytes = defer;
    }

    /// Clears the mock error (if set).
    pub fn clear_error(&mut self) {
        self.spi.borrow_mut().error = None;
    }
}

builder!(mock => MockBuilder<SpiOpts> + Debug {
    byte_delay: Option<Duration> = None,
    generator: Option<BoxedGenerator> = None,
});

impl MockBuilder {
    /// Print Tx/Rx to stdout after transfer.
    pub fn with_byte_log(mut self) -> Self {
        self.opts.log = true;
        self.opts.bytes = true;
        self
    }

    /// Use a function to provide Rx bytes.
    pub fn with_generator(mut self, generator: Generator) -> Self {
        self.generator = Some(Box::new(generator));
        self
    }

    /// Use a boxed function to provide Rx bytes.
    pub fn with_boxed_generator(mut self, generator: BoxedGenerator) -> Self {
        self.generator = Some(generator);
        self
    }

    /// Introduce a per-byte time delay for transfers.
    pub fn with_byte_delay(mut self, byte_delay: Duration) -> Self {
        self.byte_delay.replace(byte_delay);
        self
    }

    /// Create the mock device and controller.
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
