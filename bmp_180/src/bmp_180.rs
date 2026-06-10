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
        #[cfg(not(debug_assertions))]
        {
            self.twi
                .start()
                .send_address_with_write(Self::BMP_180)
                .write_data(address)
                .start()
                .send_address_with_read(Self::BMP_180)
        }
        #[cfg(debug_assertions)]
        {
            let uart = Uart::get();
            if let Err(err) = self.twi.start().expect_status(StatusCode::STARTED) {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            };
            if let Err(err) = self
                .twi
                .send_address_with_write(Self::BMP_180)
                .expect_status(StatusCode::SLA_W_ACK)
            {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
            if let Err(err) = self
                .twi
                .write_data(address)
                .expect_status(StatusCode::DATA_W_ACK)
            {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
            if let Err(err) = self.twi.start().expect_status(StatusCode::START_REPEATED) {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
            if let Err(err) = self
                .twi
                .send_address_with_read(Self::BMP_180)
                .expect_status(StatusCode::SLA_R_ACK)
            {
                uart.output().send_slice("[Error] StatusCode: ");
                uart.output().send_number(err);
                uart.output().send_slice("\n");
            }
        }
    }
    pub fn read_reg_from(&mut self, address: u8) -> u16 {
        self.to_read_init(address);
        let mut result: [u8; 2] = [0; 2];
        #[cfg(not(debug_assertions))]
        {
            if let Ok(v) = self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                result[0] = v.read_twdr();
            }
            if let Ok(v) = self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                result[1] = v.read_twdr();
            }
        }
        #[cfg(debug_assertions)]
        {
            let uart = Uart::get();
            match self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                Ok(this) => result[0] = this.read_twdr(),
                Err(err) => {
                    uart.output().send_slice("[Error] StatusCode: ");
                    uart.output().send_number(err);
                    uart.output().send_slice("\n");
                }
            }
            match self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                Ok(this) => result[1] = this.read_twdr(),
                Err(err) => {
                    uart.output().send_slice("[Error] StatusCode: ");
                    uart.output().send_number(err);
                    uart.output().send_slice("\n");
                }
            }
        }
        self.twi.stop();
        u16::from_be_bytes(result)
    }
    fn read_u16(&self) -> u16 {
        let mut result = [0; 2];
        #[cfg(debug_assertions)]
        {
            let uart = Uart::get();
            match self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                Ok(this) => {
                    result[0] = this.read_twdr();
                }
                Err(err) => {
                    uart.output().send_slice("[Error] StatusCode: ");
                    uart.output().send_number(err);
                    uart.output().send_slice("\n");
                }
            };
            match self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                Ok(this) => {
                    result[1] = this.read_twdr();
                }
                Err(err) => {
                    uart.output().send_slice("[Error] StatusCode: ");
                    uart.output().send_number(err);
                    uart.output().send_slice("\n");
                }
            };
        }
        #[cfg(not(debug_assertions))]
        {
            if let Ok(this) = self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                result[0] = this.read_twdr();
            };
            if let Ok(this) = self.twi.read_ack().expect_status(StatusCode::DATA_R_ACK) {
                result[1] = this.read_twdr();
            };
        }
        u16::from_be_bytes(result)
    }
    pub fn calibration(&mut self) -> &mut Self {
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
                9..=11 => {
                    self.m[index - 9] = self.read_u16() as i16;
                }
                _ => {}
            }
        }
        self.twi.stop();
        self
    }
    pub fn read_temperature(&mut self) -> f32 {
        self.write_to_reg(0xF4, 0x2E);
        delay_ms(5);
        let ut = self.read_reg_from(0xF6);
        #[cfg(debug_assertions)]
        {
            let uart = Uart::get();
            uart.output()
                .send_slice("[UT] = ")
                .send_number(ut)
                .send_slice_ln("");
        }
        self.calc_temperature(ut)
    }
    fn write_to_reg(&mut self, address: u8, value: u8) {
        #[cfg(debug_assertions)]
        {
            let uart = Uart::get();
            if let Err(err) = self.twi.start().expect_status(StatusCode::START_REPEATED) {
                uart.output()
                    .send_slice("[Error] StatusCode ")
                    .send_number(err)
                    .send_slice_ln(" ");
            }
            if let Err(err) = self
                .twi
                .send_address_with_write(Self::BMP_180)
                .expect_status(StatusCode::SLA_W_ACK)
            {
                uart.output()
                    .send_slice("[Error] StatusCode ")
                    .send_number(err)
                    .send_slice_ln(" ");
            }
            if let Err(err) = self
                .twi
                .write_data(address)
                .expect_status(StatusCode::DATA_W_ACK)
            {
                uart.output()
                    .send_slice("[Error] StatusCode ")
                    .send_number(err)
                    .send_slice_ln(" ");
            }
            if let Err(err) = self
                .twi
                .write_data(value)
                .expect_status(StatusCode::DATA_W_ACK)
            {
                uart.output()
                    .send_slice("[Error] StatusCode ")
                    .send_number(err)
                    .send_slice_ln("");
            }
            self.twi.stop();
        }
        #[cfg(not(debug_assertions))]
        {}
    }
    fn calc_temperature(&self, ut: u16) -> f32 {
        let x1: i32 = ((ut as i32) - (self.ac_p2[2] as i32)) * (self.ac_p2[1] as i32) >> 15;
        let x2: i32 = ((self.m[1] as i32) << 11) / (x1 + self.m[2] as i32);
        let b5 = x1 + x2;
        let t = (b5 + 8) >> 4;
        (t as f32) / 10.0
    }
}
