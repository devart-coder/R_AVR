use core::fmt::Pointer;

use nano::{
    drivers::{
        Uart,
        twi::{
            StatusCode,
            master::Master,
            twi::{PrescalerMode, Twi},
        },
    },
    utils::delay::delay_ms,
};

pub struct Bmp180 {
    pub ac_p1: [i16; 3],
    pub ac_p2: [u16; 3],
    pub b: [i16; 2],
    pub m: [i16; 3],
    pub twi: Master,
}
impl Bmp180 {
    const BMP_180: u8 = 0x77;
    pub fn new(mode: PrescalerMode, speed: u8) -> Self {
        Self {
            twi: Twi::new().prescaler_mode(mode).speed(speed).as_master(),
            ac_p1: [0; 3],
            ac_p2: [0; 3],
            b: [0; 2],
            m: [0; 3],
        }
    }

    fn to_read_init(&self, address: u8) {
        let uart = Uart::get();
        uart.settings().default();

        match self.twi.start().expect_status(StatusCode::STARTED) {
            Ok(ok) => {
                uart.output().send_slice("[Ok] start\n");
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        };
        match self
            .twi
            .send_address_with_write(Self::BMP_180)
            .expect_status(StatusCode::SLA_W_ACK)
        {
            Ok(ok) => {
                uart.output().send_slice("[Ok] AddressWrite\n");
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        }
        match self
            .twi
            .write_data(address)
            .expect_status(StatusCode::DATA_R_ACK)
        {
            Ok(ok) => {
                uart.output().send_slice("[Ok] DataWrite\n");
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        }
        match self.twi.start().expect_status(StatusCode::START_REPEATED) {
            Ok(ok) => {
                uart.output().send_slice("[Ok] Restart\n");
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        }
        match self
            .twi
            .send_address_with_read(Self::BMP_180)
            .expect_status(StatusCode::SLA_R_ACK)
        {
            Ok(ok) => {
                uart.output().send_slice("[Ok] AddressToRead\n");
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        }
    }
    fn read_reg_from(&self, address: u8) -> u16 {
        let uart = Uart::get();
        uart.settings().default();

        self.to_read_init(address);
        let mut result: [u8; 2] = [0; 2];
        if let Ok(v) = self.twi.read_ack().expect_status(StatusCode::DATA_W_ACK) {
            result[0] = v.read_twdr();
        }
        if let Ok(v) = self.twi.read_ack().expect_status(StatusCode::DATA_W_ACK) {
            result[1] = v.read_twdr();
        }
        self.twi.stop();
        u16::from_be_bytes(result)
    }
    fn read_u16(&self) -> u16 {
        let uart = Uart::get();
        let mut result = [0; 2];
        match self.twi.read_ack().expect_status(StatusCode::DATA_W_ACK) {
            Ok(this) => {
                result[0] = this.read_twdr();
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        };
        match self.twi.read_ack().expect_status(StatusCode::DATA_W_ACK) {
            Ok(this) => {
                result[1] = this.read_twdr();
            }
            Err(err) => {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        };
        u16::from_be_bytes(result)
    }

    pub fn calibration(&mut self) {
        let uart = Uart::get();
        uart.settings().default();
        self.to_read_init(0xAA);
        for index in 1..=11 {
            match index {
                4..=6 => {
                    self.ac_p2[index - 4] = self.read_u16();
                }
                1..=3 => {
                    self.ac_p1[index - 1] = self.read_u16() as i16;
                }
                7..=8 => {
                    self.b[index - 7] = self.read_u16() as i16;
                }
                9..=10 => {
                    self.m[index - 9] = self.read_u16() as i16;
                }
                11 => {
                    self.m[index - 9] = self.read_u16() as i16;
                }
                _ => {}
            }
        }
        self.twi.stop();
    }
}
