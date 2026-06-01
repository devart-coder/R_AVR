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
    let master = twi.prescaler_mode(PrescalerMode::_1).speed(72).as_master();
    let master = match master.start() {
        Ok(s) => {
            uart.output().send_slice("Start... [OK]\n");
            s
        }
        Err(err) => {
            uart.output().send_slice("Start... [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    let master = match master.send_address_with_write(BMP_180) {
        Ok(s) => {
            uart.output().send_slice("AddressWrite... [OK]\n");
            s
        }
        Err(err) => {
            uart.output().send_slice("AddressWrite... [Error]\n");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    let master = match master.write_data(0xF4) {
        Ok(this) => {
            uart.output().send_slice("WriteData... [OK]\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("WriteData... [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    let master = match master.write_data(0x2E) {
        Ok(this) => {
            uart.output().send_slice("WriteData... [OK]\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("WriteData... [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };

    master.stop();
    let master = match master.start() {
        Ok(this) => {
            uart.output().send_slice("Restart... [OK]\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("Restart... [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    let master = match master.send_address_with_read(BMP_180) {
        Ok(this) => {
            uart.output().send_slice("WriteAddressWithRead... [OK]\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("WriteAddressWithRead... [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    let master = match master.restart() {
        Ok(this) => {
            uart.output().send_slice("Restart... [OK]\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("Restart... [Error] ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    master.stop();
}
