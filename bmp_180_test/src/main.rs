#![no_std]
#![no_main]
use bmp_180::bmp_180::Bmp180;
use nano::{
    F_CPU,
    drivers::{BaudRate, Uart, twi::twi::PrescalerMode},
    utils::delay::{self, delay_ms},
};
#[unsafe(no_mangle)]
fn main() {
    let uart = Uart::get();
    uart.settings().default().baud_rate(BaudRate::_9600);

    let mut bmp = Bmp180::new(100_000);
    loop {
        let t = bmp.read_temperature();
        let value = bmp.read_pressure(0);
        let p = value.to_pascal();
        let mm_hg = value.to_mm_hg();
        uart.output()
            .send_slice("[Temperature] = ")
            .send_number(t as i32)
            .send_slice_ln(" C")
            .send_slice("[Pressure] = ")
            .send_number(mm_hg)
            .send_slice(" mmHg ( ")
            .send_number(p)
            .send_slice_ln(" P )");
        delay_ms(5000);
    }
}
