use crate::{
    drivers::{Uart, uart::uart},
    mcu::registers::{
        twcr::{Twcr, TwcrBuilder},
        twdr::Twdr,
        twsr::Twsr,
    },
    utils::register::Register,
};

pub struct Twi {
    twcr: Twcr,
    twdr: Register,
    twsr: Register,
}
impl Twi {
    const READ: u8 = 1;
    const WRITE: u8 = 0;
    pub fn new() -> Self {
        Self {
            twcr: Twcr::new(),
            twdr: Register::new(Twdr::address()),
            twsr: Register::new(Twsr::address()),
        }
    }
    fn _check_twint_flag_set(self) -> Self {
        let value = TwcrBuilder(0).twint().set_bit().build();
        while (self.twcr.read() & value) == 0 {}
        self
    }
    fn _check_status_code(&self, status: u8) -> Result<u8, u8> {
        match self.twsr.read() & 0xF8 == status {
            true => Ok(status),
            false => Err(0),
        }
    }

    pub fn start(self) -> Self {
        self.twcr
            .modify(|v| v.twen().set_bit().twsta().set_bit().twint().set_bit());
        let s = self._check_twint_flag_set();
        match s._check_status_code(StatusCode::STARTED) {
            Err(r) => {
                let uart = uart::Uart::get();
                uart.output.send_slice("[Error] ");
                uart.output.send_slice(str::from_utf8(&[r]).unwrap());
            }
            Ok(_) => (),
        };
        s
    }
    pub fn stop(self) -> Self {
        self.twcr
            .modify(|v| v.twen().set_bit().twsto().set_bit().twint().set_bit());
        self
    }
    pub fn send_address_with_write(self, val: u8) -> Self {
        self.write_data(val << 1 | Self::WRITE)
            ._check_twint_flag_set()
    }
    pub fn send_address_with_read(self, val: u8) -> Self {
        self.write_data(val << 1 | Self::READ)
            ._check_twint_flag_set()
    }
    pub fn write_data(self, val: u8) -> Self {
        self.twdr.write(val);
        self.twcr.modify(|v| v.twint().set_bit().twen().set_bit());
        self
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

    const SLAW_ACK: u8 = 0x18;
    const SLAW_NACK: u8 = 0x20;

    const LOST: u8 = 0x38;
}
