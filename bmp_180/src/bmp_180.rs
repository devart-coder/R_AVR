use core::ops::Div;

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
    ac_p1: [i16; 3],
    ac_p2: [u16; 3],
    b: [i16; 2],
    m: [i16; 3],
    b5: i32,
    pub twi: Master,
}
impl Bmp180 {
    const BMP_180: u8 = 0x77;
    pub fn new(speed: u32) -> Self {
        let mut this = Self {
            twi: Twi::new()
                .prescaler_mode(PrescalerMode::_1)
                .speed(speed)
                .as_master(),
            ac_p1: [0; 3],
            ac_p2: [0; 3],
            b: [0; 2],
            m: [0; 3],
            b5: i32::default(),
        };
        this.calibration();
        this
    }
    fn to_read_init(&self, address: u8) {
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
    fn read_u16(&self) -> u16 {
        let mut result = [0; 2];
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
        u16::from_be_bytes(result)
    }
    fn write_to_reg(&mut self, address: u8, value: u8) {
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
    fn temp_calc(&mut self, ut: u16) -> f32 {
        let x1: i32 = ((ut as i32).wrapping_sub(self.ac_p2[2] as i32))
            .wrapping_mul((self.ac_p2[1] as i32))
            / 32768;
        let d = x1.wrapping_add(self.m[2] as i32);
        let x2: i32 = ((self.m[1] as i32) << 11).checked_div(d).unwrap_or(0);
        self.b5 = x1.wrapping_add(x2);
        let t = self.b5.wrapping_add(8).wrapping_div(16);
        (t as f32).div(10.0)
    }
    fn pressure_calc(&self, up: u16, oss: u8) -> i32 {
        let b6: i32 = self.b5.wrapping_sub(4000);
        let x1 = ((self.b[1] as i32).wrapping_mul(b6.wrapping_mul(b6) / 4096)) / 2048;
        let x2 = (self.ac_p1[1] as i32).wrapping_mul(b6) / 2048;
        let x3 = x1.wrapping_add(x2);
        let b3 = (((self.ac_p1[0] as i32).wrapping_mul(4).wrapping_add(x3) << oss) + 2) / 4;

        let x1 = (self.ac_p1[2] as i32).wrapping_mul(b6) >> 13;
        let x2 = (self.b[0] as i32).wrapping_mul(b6.wrapping_mul(b6) >> 12) >> 16;
        let x3 = x1.wrapping_add(x2).wrapping_add(2) >> 2;
        let b4 = (self.ac_p2[0] as u32).wrapping_mul(x3.wrapping_add(32768) as u32) / 32768;
        let b7 = (up as u32)
            .wrapping_sub(b3 as u32)
            .wrapping_mul(50000 >> oss);

        let mut p: i32 = 0;
        if b4 != 0 {
            if b7 < 0x80000000u32 {
                p = ((b7.wrapping_mul(2)) / b4) as i32;
            } else {
                p = ((b7 / b4).wrapping_mul(2)) as i32;
            }
        }

        let x1 = (p >> 8).wrapping_mul(p >> 8);
        let x1 = x1.wrapping_mul(3038) >> 16;
        let x2 = p.wrapping_mul(-7357) >> 16;
        p = p.wrapping_add(x1.wrapping_add(x2).wrapping_add(3791) >> 4);
        p
    }
    fn read_reg_from(&mut self, address: u8) -> u16 {
        self.to_read_init(address);
        self.read_u16()
    }
    fn calibration(&mut self) -> &mut Self {
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
    pub fn set_prescaler(&self, mode: PrescalerMode) -> &Self {
        //todo!()
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
        self.temp_calc(ut)
    }
    pub fn read_pressure(&mut self, oss: u8) -> PressureConversion {
        self.write_to_reg(0xF4, 0x34);
        delay_ms(5);
        let up = self.read_reg_from(0xF6);
        #[cfg(debug_assertions)]
        {
            let uart = Uart::get();
            uart.output()
                .send_slice("[UP] = ")
                .send_number(up)
                .send_slice_ln("");
        }
        PressureConversion::new(self.pressure_calc(up, oss))
    }
}
pub struct PressureConversion {
    pascal_pressure: i32,
}
impl PressureConversion {
    fn new(v: i32) -> Self {
        Self { pascal_pressure: v }
    }
    pub fn to_pascal(&self) -> i32 {
        self.pascal_pressure
    }
    pub fn to_mm_hg(&self) -> i32 {
        (self.pascal_pressure * 100) / 13332
    }
}
