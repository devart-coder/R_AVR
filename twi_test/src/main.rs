#![no_std]
#![no_main]
use nano::{
    drivers::{
        Uart,
        twi::{
            StatusCode,
            twi::{PrescalerMode, Twi},
        },
    },
    mcu::pins::Pins,
};

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
    match master.start().expect_status(StatusCode::STARTED) {
        Ok(s) => {
            uart.output().send_slice("[OK] Start\n");
            s
        }
        Err(err) => {
            uart.output().send_slice("[Error] Start: ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    match master
        .send_address_with_write(BMP_180)
        .expect_status(StatusCode::SLA_W_ACK)
    {
        Ok(s) => {
            uart.output().send_slice("[OK] WriteAddress\n");
            s
        }
        Err(err) => {
            uart.output().send_slice("[Error] WriteAddress: ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    match master
        .write_data(0xAA)
        .expect_status(StatusCode::DATA_R_ACK)
    {
        Ok(this) => {
            uart.output().send_slice("[OK] WriteData\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("[Error] WriteData: ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    match master.start().expect_status(StatusCode::START_REPEATED) {
        Ok(this) => {
            uart.output().send_slice("[OK] Restart\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("[Error] Restart: ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    match master
        .send_address_with_read(BMP_180)
        .expect_status(StatusCode::SLA_R_ACK)
    {
        Ok(this) => {
            uart.output().send_slice("[OK] SWA_R\n");
            this
        }
        Err(err) => {
            uart.output().send_slice("[Error] SWA_R: ");
            uart.output().send_slice(str::from_utf8(&[err]).unwrap());
            uart.output().send_slice("\n");
            return;
        }
    };
    // match master.read().expect_status(StatusCode::DATA_W_ACK) {
    //     Ok(r) => {
    //         uart.output().send_slice("[OK] Get from 0xAA: ");
    //         uart.output()
    //             .send_slice(str::from_utf8(&[r.read_twdr()]).unwrap());
    //         uart.output().send_slice("\n");
    //     }
    //     Err(error) => {
    //         uart.output().send_slice("[Error] Get from 0xAA: ");
    //         uart.output().send_slice(str::from_utf8(&[error]).unwrap());
    //         uart.output().send_slice("\n");
    //     }
    // }
    let result = master.read_array::<4>();
    uart.output()
        .send_slice(str::from_utf8(&[result.len() as u8]).unwrap());
    for i in result {
        uart.output().send_slice(str::from_utf8(&[i]).unwrap());
        uart.output().send_slice("\n");
    }
    //     .expect_status(StatusCode::DATA_W_ACK); {
    //     Ok(this) => {
    //         uart.output().send_slice("[OK] DATA_R: ");
    //         uart.output()
    //             .send_slice(str::from_utf8(&[this.read_twdr()]).unwrap());
    //         this
    //     }
    //     Err(err) => {
    //         uart.output().send_slice("[Error] DATA_R: ");
    //         uart.output().send_slice(str::from_utf8(&[err]).unwrap());
    //         uart.output().send_slice("\n");
    //         return;
    //     }
    // };
    master.stop();
}
