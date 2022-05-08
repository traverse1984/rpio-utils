use crate::Transfer;
use std::{borrow::ToOwned, cell::RefCell, format, println, rc::Rc, string::String, vec::Vec};

/// Intercepts [`Transfer<u8>`](Transfer), providing logging capabilities.
#[derive(Debug)]
pub struct Spi<S: Transfer<u8>> {
    name: String,
    spi: S,
    opts: Rc<RefCell<SpiOpts>>,
}

impl<S: Transfer<u8>> Spi<S> {
    pub fn new(name: String, spi: S, opts: Rc<RefCell<SpiOpts>>) -> Self {
        Self { name, spi, opts }
    }

    /// Set whether events are printed to stdout.
    pub fn set_log(&self, log: bool) {
        self.opts.borrow_mut().log = log;
    }

    /// Set whether Tx/Rx bytes are printed (when logging is enabled).
    pub fn set_log_bytes(&self, bytes: bool) {
        self.opts.borrow_mut().bytes = bytes;
    }
}

impl<S: Transfer<u8>> Transfer<u8> for Spi<S> {
    type Error = S::Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        let opts = *self.opts.borrow();

        if opts.log {
            println!("{} -> Start transfer ({} bytes)", self.name, words.len());
            let tx = words.to_vec();
            let result = self.spi.transfer(words);

            match result {
                Ok(rx) => {
                    if opts.bytes {
                        print_summary(&tx, rx)
                    }

                    println!("{} -> Transfer complete", self.name);
                }
                Err(_) => println!("{} -> Error (transfer failed)", self.name),
            };

            result
        } else {
            self.spi.transfer(words)
        }
    }
}

/// Options for constructing an SPI intercept.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpiOpts {
    pub log: bool,
    pub bytes: bool,
}

impl SpiOpts {
    pub fn new() -> Self {
        Self {
            log: true,
            bytes: false,
        }
    }
}

builder!(InterceptBuilder<SpiOpts> + Debug, Clone {});

impl InterceptBuilder {
    pub fn with_byte_log(mut self) -> Self {
        self.opts.log = true;
        self.opts.bytes = true;
        self
    }

    pub fn init<S: Transfer<u8>>(self, spi: S) -> Spi<S> {
        Spi::new(self.name, spi, Rc::new(RefCell::new(self.opts)))
    }
}

fn printable_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}

fn print_summary(tx: &[u8], rx: &[u8]) {
    for (chunk, (tx, rx)) in tx.chunks(16).zip(rx.chunks(16)).enumerate() {
        let range = format!("{}-{}", chunk * 16, (chunk * 16) + 16.min(tx.len()));
        println!("{: >12} --> {} -->", range, printable_bytes(tx));
        println!("{: >12} <-- {} <--", "", printable_bytes(rx))
    }
}
