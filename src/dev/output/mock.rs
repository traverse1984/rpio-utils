use super::intercept::{Pin, PinOpts};
use embedded_hal::digital::v2::OutputPin;
use std::{borrow::ToOwned, cell::RefCell, rc::Rc, string::String, thread, time::Duration};

/// State interface for mock output pin.
#[derive(Debug)]
pub struct MockPin {
    dev: Rc<RefCell<MockPinDevice>>,
}

impl MockPin {
    fn new(dev: Rc<RefCell<MockPinDevice>>) -> Self {
        Self { dev }
    }
}

impl OutputPin for MockPin {
    type Error = PinError;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.dev
            .borrow_mut()
            .set_value_unless(true, PinError::SetHigh)
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.dev
            .borrow_mut()
            .set_value_unless(false, PinError::SetLow)
    }
}

/// An enum of mock output pin errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinError {
    SetHigh,
    SetLow,
}

/// Holds the underlying state shared by [MockPin] and [PinControl].
#[derive(Debug)]
pub struct MockPinDevice {
    opts: Rc<RefCell<PinOpts>>,
    value: bool,
    delay: Option<Duration>,
    error: Option<PinError>,
}

impl MockPinDevice {
    fn new(opts: Rc<RefCell<PinOpts>>) -> Self {
        Self {
            opts,
            value: true,
            delay: None,
            error: None,
        }
    }

    fn set_value_unless(&mut self, value: bool, unless: PinError) -> Result<(), PinError> {
        if let Some(delay) = self.delay {
            thread::sleep(delay);
        }

        match self.error {
            Some(error) if error == unless => Err(self.error.take().unwrap()),
            _ => {
                self.value = value;
                Ok(())
            }
        }
    }
}

/// Developer controls for mock output pin.
#[derive(Debug)]
pub struct PinControl {
    pin: Rc<RefCell<MockPinDevice>>,
}

impl PinControl {
    fn new(pin: Rc<RefCell<MockPinDevice>>) -> Self {
        Self { pin }
    }

    /// Set whether events are printed to stdout.
    pub fn set_log(&self, log: bool) -> &Self {
        self.pin.borrow_mut().opts.borrow_mut().log = log;
        self
    }

    /// Set the time delay for state change calls.
    pub fn set_delay(&self, duration: Duration) -> &Self {
        self.pin.borrow_mut().delay = Some(duration);
        self
    }

    /// Clear the time delay for state change calls.
    pub fn clear_delay(&self) -> &Self {
        self.pin.borrow_mut().delay = None;
        self
    }

    /// Set a mock error. The next time this error could occur - it does.
    pub fn set_error(&self, error: PinError) -> &Self {
        self.pin.borrow_mut().error = Some(error);
        self
    }

    /// Clear the mock error (if set).
    pub fn clear_error(&self) -> &Self {
        self.pin.borrow_mut().error = None;
        self
    }

    /// Get the current value of the pin (as bool).
    pub fn get_value(&self) -> bool {
        self.pin.borrow().value
    }
}

builder!(MockBuilder<PinOpts> + Clone, Debug {
    delay: Option<Duration> = None,
});

impl MockBuilder {
    /// Introduce a time delay for state change calls.
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay.replace(delay);
        self
    }

    /// Create the mock output pin and controller.
    pub fn init(self) -> (Pin<MockPin>, PinControl) {
        let opts = Rc::new(RefCell::new(self.opts));
        let dev = Rc::new(RefCell::new(MockPinDevice::new(opts.clone())));
        let control = PinControl::new(dev.clone());
        let pin = Pin::new(self.name, MockPin::new(dev), opts);

        if let Some(delay) = self.delay {
            control.set_delay(delay);
        }

        (pin, control)
    }
}
