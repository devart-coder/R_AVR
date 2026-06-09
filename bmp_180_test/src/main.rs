#![no_std]
#![no_main]
use bmp_180::bmp_180::Bmp180;
use nano::drivers::Uart;
#[unsafe(no_mangle)]
fn main() {
    let uart = Uart::get();
    uart.settings().default();

    let mut bmp = Bmp180::new(nano::drivers::twi::twi::PrescalerMode::_1, 72);
    bmp.calibration();
    uart.output().send_slice("AC1: ");
    uart.output().send_number(bmp.ac_p1[0]);
    uart.output().send_slice("\n");

    uart.output().send_slice("AC2: ");
    uart.output().send_number(bmp.ac_p1[1]);
    uart.output().send_slice("\n");

    uart.output().send_slice("AC3: ");
    uart.output().send_number(bmp.ac_p1[2]);
    uart.output().send_slice("\n");

    uart.output().send_slice("AC4: ");
    uart.output().send_number(bmp.ac_p2[0]);
    uart.output().send_slice("\n");

    uart.output().send_slice("AC5: ");
    uart.output().send_number(bmp.ac_p2[1]);
    uart.output().send_slice("\n");

    uart.output().send_slice("AC6: ");
    uart.output().send_number(bmp.ac_p2[2]);
    uart.output().send_slice("\n");

    uart.output().send_slice("B0: ");
    uart.output().send_number(bmp.b[0]);
    uart.output().send_slice("\n");

    uart.output().send_slice("B1: ");
    uart.output().send_number(bmp.b[1]);
    uart.output().send_slice("\n");

    uart.output().send_slice("M0: ");
    uart.output().send_number(bmp.m[0]);
    uart.output().send_slice("\n");

    uart.output().send_slice("M1: ");
    uart.output().send_number(bmp.m[1]);
    uart.output().send_slice("\n");

    uart.output().send_slice("M2: ");
    uart.output().send_number(bmp.m[2]);
    uart.output().send_slice("\n");
}
