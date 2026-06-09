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
    const CALIBRATION_SIZE: usize = 11;
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
        if let Ok(v) = self.twi.read().expect_status(StatusCode::DATA_W_ACK) {
            result[0] = v.read_twdr();
        }
        if let Ok(v) = self.twi.read().expect_status(StatusCode::DATA_W_ACK) {
            result[1] = v.read_twdr();
        }
        self.twi.stop();
        u16::from_be_bytes(result)
    }

    fn read_array(&mut self, address: u8) {
        let uart = Uart::get();
        uart.settings().default();
        self.to_read_init(address);

        for index in (1..=10) {
            uart.output().send_slice("[Index] ");
            uart.output().send_u8(index as u8);
            uart.output().send_slice("\n");
            match index {
                4..=6 => {
                    self.ac_p2[index] = match self.twi.read().expect_status(StatusCode::DATA_W_ACK)
                    {
                        Ok(this) => {
                            uart.output().send_slice("[Ok] RegisterReaded\n");
                            this.read_u16()
                        }
                        Err(err) => {
                            uart.output().send_slice("[Error] StatusCode: ");
                            uart.output().send_u8(err);
                            uart.output().send_slice("\n");
                            return;
                        }
                    };
                }
                _ => {
                    let res = match self.twi.read().expect_status(StatusCode::DATA_R_ACK) {
                        Ok(this) => {
                            uart.output().send_slice("[Ok] RegisterReaded\n");
                            this.read_u16()
                        }
                        Err(err) => {
                            uart.output().send_slice("[Error] StatusCode: ");
                            uart.output().send_u8(err);
                            uart.output().send_slice("\n");
                            return;
                        }
                    };
                }
            }
        }
    }

    pub fn calibration(&mut self) {
        self.read_array(0xAA);
        // let mut offset = 0;
        // self.ac_p2[0] = self.read_reg_from(0xAA);
        // self.ac_p2[1] = self.read_reg_from(0xAA + 4 * 2);
        // self.ac_p2[2] = self.read_reg_from(0xAA + 5 * 2);
        // for address in (0xAA..=0xBE).step_by(2) {
        // offset += 1;
        // let mut register = match offset {
        //     1 => self.ac1,
        //     2 => self.ac2,
        //     3 => self.ac3,
        //     7 => self.b1,
        //     8 => self.b2,
        //     9 => self.mb,
        //     10 => self.mc,
        //     11 => self.md,
        //     _ => 0,
        // };
        // match address {
        // 0xB0 => self.ac4 = self.read_from_reg_u16(address),
        // 0xB2 => self.ac5 = self.read_from_reg_u16(address),
        // 0xB4 => self.ac6 = self.read_from_reg_u16(address),
        // _ => {} // _ => {
        //     register = self.read_from_reg_u16(address) as i16;
        // }
        // }
        // }
    }
}
struct ArrayWrapper {}
impl ArrayWrapper {
    pub fn from_address(self, Address: u8) -> Self {
        self
    }
    pub fn size<const SIZE: usize>(self) -> Self {
        self
    }
    fn result(&self) {}
}
