#![no_std]
#![no_main]
use bmp_180::bmp_180::Bmp180;
use nano::{
    drivers::{BaudRate, Uart, twi::twi::PrescalerMode},
    utils::delay::{self, delay_ms},
};
#[unsafe(no_mangle)]
fn main() {
    let uart = Uart::get();
    uart.settings().default().baud_rate(BaudRate::_9600);

    let mut bmp = Bmp180::new(PrescalerMode::_1, 72);
    bmp.calibration();
    loop {
        let result = bmp.read_temperature();
        uart.output()
            .send_slice("[T] = ")
            .send_number(result as i32)
            .send_slice_ln("");
        delay_ms(2000);
    }
}
