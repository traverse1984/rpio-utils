extern crate rpio_utils;

use rpio_utils::{dev::*, *};

fn main() {
    let (spi, sctrl) = Mock::spi("SPI")
        .with_byte_log()
        .with_generator(|x| x.to_vec())
        .init();

    let (pin, ..) = Mock::pin("CS").init();

    let mut spi = Transport::new(spi).with_chip_select(pin).init();

    let mut x = [
        1, 2, 3, 4, 1, 2, 3, 4, 8, 8, 8, 8, 9, 9, 9, 9, 23, 34, 45, 56, 56, 67, 67, 78, 78, 89, 89,
        90, 5, 1, 2, 3,
    ];

    sctrl
        .set_error(SpiError::Transfer)
        .set_error_defer_bytes(33);

    spi.transfer(&mut x).ok();
    spi.transfer(&mut [1]).ok();
    spi.transfer(&mut [2]).ok();
}
