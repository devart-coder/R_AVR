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
    fn _check_twint_flag_set(self) -> Self {
        let value = TwcrBuilder(0).twint().set_bit().build();
        while (self.twcr.read() & value) == 0 {}
        self
    }
    fn _check_status_code(self, status: u8) -> Result<Self, u8> {
        match self.twsr.read() & 0xF8 == status {
            true => Ok(self),
            false => Err(status),
        }
    }

    pub fn start(self) -> Result<Self, u8> {
        self.twcr
            .modify(|v| v.twen().set_bit().twsta().set_bit().twint().set_bit());
        let this = self._check_twint_flag_set();
        this._check_status_code(StatusCode::STARTED)
    }
    pub fn stop(self) {
        self.twcr
            .modify(|v| v.twen().set_bit().twsto().set_bit().twint().set_bit());
    }
    pub fn send_address_with_write(self, val: u8) -> Result<Self, u8> {
        let this = self
            .write_data(val << 1 | Self::WRITE)
            ._check_twint_flag_set();
        this._check_status_code(StatusCode::SLA_W_ACK)
    }
    pub fn send_address_with_read(self, val: u8) -> Result<Self, u8> {
        let this = self
            .write_data(val << 1 | Self::READ)
            ._check_twint_flag_set();
        this._check_status_code(StatusCode::SLA_R_ACK)
    }
    pub fn write_data(self, val: u8) -> Self {
        self.twdr.write(val);
        self.twcr.modify(|v| v.twint().set_bit().twen().set_bit());
        self._check_twint_flag_set()
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
