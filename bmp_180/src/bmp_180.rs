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
    pub ac1: i16,
    pub ac2: i16,
    pub ac3: i16,
    pub ac4: u16,
    pub ac5: u16,
    pub ac6: u16,
    pub b1: i16,
    pub b2: i16,
    pub mb: i16,
    pub mc: i16,
    pub md: i16,
    pub twi: Master,
}
impl Bmp180 {
    const BMP_180: u8 = 0x77;
    pub fn new(mode: PrescalerMode, speed: u8) -> Self {
        Self {
            twi: Twi::new().prescaler_mode(mode).speed(speed).as_master(),
            ac1: 0,
            ac2: 0,
            ac3: 0,
            ac4: 0,
            ac5: 0,
            ac6: 0,
            b1: 0,
            b2: 0,
            mb: 0,
            mc: 0,
            md: 0,
        }
    }
    fn read_from_reg_u16(&self, address: u8) -> u16 {
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
        //Todo::Read_Registers;
        // let result = self.twi.read_u16();
        // self.twi.stop();
        // delay_ms(500);
        result
    }
    pub fn calibration(&mut self) {
        // let mut offset = 0;
        self.ac4 = self.read_from_reg_u16(0xAA + 4 * 2);
        self.ac5 = self.read_from_reg_u16(0xAA + 5 * 2);
        self.ac6 = self.read_from_reg_u16(0xAA + 6 * 2);
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
