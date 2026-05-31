#![no_std]
#![no_main]
use nano::{
    drivers::{
        Uart,
        twi::twi::{PrescalerMode, Twi},
    },
    mcu::pins::Pins,
};

use core::result::Result::{Err, Ok};

#[unsafe(no_mangle)]
fn main() {
    const BMP_180: u8 = 0x77;
    let pins = Pins::take();
    pins.a4.into_output();
    pins.a5.into_output();

    let uart = Uart::get();
    uart.settings().default();

    let twi = Twi::new();
    let master = twi.prescaler_mode(PrescalerMode::_1).speed(123).as_master();
    let master = match master.start() {
        Ok(s) => {
            uart.output().send_slice("Start [OK]\n");
            s
        }
        Err(err) => {
            uart.output().send_slice("[Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    let master = match master.send_address_with_write(BMP_180) {
        Ok(s) => {
            uart.output().send_slice("AddressWrite [OK]");
            s
        }
        Err(err) => {
            uart.output().send_slice("AddressWrite [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    master.write_data(0xff).stop();
}
