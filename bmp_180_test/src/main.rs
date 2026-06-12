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
        let t = bmp.read_temperature();
        let p = bmp.read_pressure(0);
        let mm_hg_x10 = (p * 100) / 13332;
        let p_whole = mm_hg_x10;
        // let p_fract = mm_hg_x10 % 10;
        uart.output()
            .send_slice("[Temperature] = ")
            .send_number(t as i32)
            .send_slice_ln(" C")
            .send_slice("[Pressure] = ")
            .send_number(p)
            .send_slice(" P ( ")
            .send_number(p_whole)
            // .send_slice(".")
            // .send_number(p_fract)
            .send_slice_ln(" mmHg )");
        delay_ms(5000);
    }
}
