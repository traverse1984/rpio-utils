use super::{
    error::PinError,
    intercept::{Pin, PinOpts},
};
use embedded_hal::digital::v2::OutputPin as HalOutputPin;
use std::{borrow::ToOwned, cell::RefCell, rc::Rc, string::String, thread, time::Duration};

#[derive(Debug)]
pub struct MockPin {
    dev: Rc<RefCell<MockPinDevice>>,
}

impl MockPin {
    pub fn new(dev: Rc<RefCell<MockPinDevice>>) -> Self {
        Self { dev }
    }
}

impl HalOutputPin for MockPin {
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

#[derive(Debug)]
pub struct MockPinDevice {
    opts: Rc<RefCell<PinOpts>>,
    value: bool,
    delay: Option<Duration>,
    error: Option<PinError>,
}

impl MockPinDevice {
    pub fn new(opts: Rc<RefCell<PinOpts>>) -> Self {
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

#[derive(Debug)]
pub struct PinControl {
    pin: Rc<RefCell<MockPinDevice>>,
}

impl PinControl {
    pub fn new(pin: Rc<RefCell<MockPinDevice>>) -> Self {
        Self { pin }
    }

    pub fn set_log(&mut self, log: bool) {
        self.pin.borrow_mut().opts.borrow_mut().log = log;
    }

    pub fn set_delay(&mut self, duration: Duration) {
        self.pin.borrow_mut().delay = Some(duration);
    }

    pub fn clear_delay(&mut self) {
        self.pin.borrow_mut().delay = None;
    }

    pub fn set_error(&mut self, error: PinError) {
        self.pin.borrow_mut().error = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.pin.borrow_mut().error = None;
    }

    pub fn get_value(&self) -> bool {
        self.pin.borrow().value
    }
}

builder!(mock => MockBuilder<PinOpts> + Clone, Debug {
    delay: Option<Duration> = None,
});

impl MockBuilder {
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay.replace(delay);
        self
    }

    pub fn init(self) -> (Pin<MockPin>, PinControl) {
        let opts = Rc::new(RefCell::new(self.opts));
        let dev = Rc::new(RefCell::new(MockPinDevice::new(opts.clone())));
        let mut control = PinControl::new(dev.clone());
        let pin = Pin::new(self.name, MockPin::new(dev), opts);

        if let Some(delay) = self.delay {
            control.set_delay(delay);
        }

        (pin, control)
    }
}
