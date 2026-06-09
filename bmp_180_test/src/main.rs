#![no_std]
#![no_main]
#[unsafe(no_mangle)]
use bmp_180::bmp_180::Bmp180;
use nano::drivers::Uart;
#[unsafe(no_mangle)]
fn main() {
    let uart = Uart::get();
    uart.settings().default();

    let mut bmp = Bmp180::new(nano::drivers::twi::twi::PrescalerMode::_1, 72);
    bmp.calibration();
    uart.output().send_slice("AC4: ");
    uart.output().send_u16_be(bmp.ac_p2[0]);
    uart.output().send_slice("\n");
    uart.output().send_slice("AC5: ");
    uart.output().send_u16_be(bmp.ac_p2[1]);
    uart.output().send_slice("\n");
    uart.output().send_slice("AC6: ");
    uart.output().send_u16_be(bmp.ac_p2[2]);
    uart.output().send_slice("\n");
}
