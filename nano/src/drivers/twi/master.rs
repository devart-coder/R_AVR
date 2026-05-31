use crate::{
    mcu::registers::{
        twcr::{Twcr, TwcrBuilder},
        twdr::Twdr,
        twsr::Twsr,
    },
    utils::register::Register,
};

pub struct Master {
    twcr: Twcr,
    twdr: Register,
    twsr: Twsr,
}

impl Master {
    const READ: u8 = 1;
    const WRITE: u8 = 0;

    pub fn new() -> Self {
        Self {
            twcr: Twcr::new(),
            twdr: Register::new(Twdr::address()),
            twsr: Twsr::new(),
        }
    }
    fn wait_for_send(self) -> Self {
        let value = TwcrBuilder(0).twint().set_bit().build();
        while (self.twcr.read() & value) == 0 {}
        self
    }
    fn check_status_code(self, status: u8) -> Result<Self, u8> {
        match self.twsr.read() & 0xF8 == status {
            true => Ok(self),
            false => Err(self.twsr.read() & 0xF8),
        }
    }

    pub fn start(self) -> Result<Self, u8> {
        self.twcr
            .modify(|v| v.twen().set_bit().twsta().set_bit().twint().set_bit());
        self.wait_for_send().check_status_code(StatusCode::STARTED)
    }
    pub fn stop(self) {
        self.twcr
            .modify(|v| v.twen().set_bit().twsto().set_bit().twint().set_bit());
    }
    pub fn send_address_with_write(self, val: u8) -> Result<Self, u8> {
        self.write_byte(val << 1 | Self::WRITE)
            .check_status_code(StatusCode::SLA_W_ACK)
    }
    pub fn send_address_with_read(self, val: u8) -> Result<Self, u8> {
        self.write_byte(val << 1 | Self::READ)
            .check_status_code(StatusCode::SLA_R_ACK)
    }
    fn write_byte(self, v: u8) -> Self {
        self.twdr.write(v);
        self.twcr.modify(|v| v.twint().set_bit().twen().set_bit());
        self.wait_for_send()
    }
    pub fn write_data(self, val: u8) -> Result<Self, u8> {
        self.write_byte(val)
            .check_status_code(StatusCode::DATA_RACK)
    }
}
pub enum StatusCode {}
impl StatusCode {
    const STARTED: u8 = 0x08;
    const START_REPEATED: u8 = 0x10;

    const DATA_WACK: u8 = 0x50;
    const DATA_WNACK: u8 = 0x58;

    const DATA_RACK: u8 = 0x28;
    const DATA_RNACK: u8 = 0x30;

    const SLA_W_ACK: u8 = 0x18;
    const SLA_W_NACK: u8 = 0x20;

    const SLA_R_ACK: u8 = 0x40;
    const SLA_R_NACK: u8 = 0x48;

    const LOST: u8 = 0x38;
}
