use embedded_hal::digital::v2::OutputPin;
use std::{borrow::ToOwned, cell::RefCell, println, rc::Rc, string::String};

/// Intercepts [`OutputPin`](OutputPin), providing logging.
pub struct Pin<P: OutputPin> {
    name: String,
    pin: P,
    opts: Rc<RefCell<PinOpts>>,
}

impl<P: OutputPin> Pin<P> {
    pub fn new(name: String, pin: P, opts: Rc<RefCell<PinOpts>>) -> Self {
        Self { name, pin, opts }
    }

    /// Set whether events are printed to stdout.
    pub fn set_log(&mut self, log: bool) {
        self.opts.borrow_mut().log = log;
    }
}

impl<P: OutputPin> OutputPin for Pin<P> {
    type Error = P::Error;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let result = self.pin.set_high();
        if self.opts.borrow().log {
            match result {
                Ok(_) => println!("{} -> high", self.name),
                Err(_) => println!("{} -> Error (not set high)", self.name),
            };
        }

        result
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        if self.opts.borrow().log {
            self.pin
                .set_low()
                .map(|_| println!("{} -> low", self.name))
                .map_err(|err| {
                    println!("{} -> Error (not set low)", self.name);
                    err
                })
        } else {
            self.pin.set_low()
        }
    }
}

/// Options for constructing an output pin intercept.
#[derive(Debug, Clone, Copy, Default)]
pub struct PinOpts {
    pub log: bool,
}

impl PinOpts {
    pub fn new() -> Self {
        Self { log: true }
    }
}

builder!(InterceptBuilder<PinOpts> + Clone, Debug {});

impl InterceptBuilder {
    pub fn init<P: OutputPin>(self, pin: P) -> Pin<P> {
        Pin::new(self.name, pin, Rc::new(RefCell::new(self.opts)))
    }
}
