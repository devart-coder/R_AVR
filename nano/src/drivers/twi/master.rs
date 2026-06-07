use crate::{
    drivers::twi::StatusCode,
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
    pub fn expect_status(self, status: u8) -> Result<Self, u8> {
        match self.twsr.read() & 0xF8 == status as u8 {
            true => Ok(self),
            false => Err(self.twsr.read() & 0xF8),
        }
    }
    pub fn start(self) -> Self {
        self.twcr
            .modify(|v| v.twen().set_bit().twsta().set_bit().twint().set_bit());
        self.wait_for_send()
    }
    pub fn restart(self) -> Self {
        self.twcr
            .modify(|v| v.twen().set_bit().twsta().set_bit().twint().set_bit());
        self.wait_for_send()
    }
    pub fn stop(&self) {
        self.twcr
            .modify(|v| v.twen().set_bit().twsto().set_bit().twint().set_bit());
    }
    pub fn send_address_with_write(self, val: u8) -> Self {
        self.write_byte(val << 1 | Self::WRITE)
    }
    pub fn send_address_with_read(self, val: u8) -> Self {
        self.write_byte(val << 1 | Self::READ)
    }
    fn write_byte(self, v: u8) -> Self {
        self.twdr.write(v);
        self.twcr.modify(|v| v.twint().set_bit().twen().set_bit());
        self.wait_for_send()
    }
    pub fn write_data(self, val: u8) -> Self {
        self.write_byte(val)
    }
    pub fn read_twdr(&self) -> u8 {
        self.twdr.read()
    }
    pub fn read_with_ack(self) -> Self {
        self.twcr
            .modify(|b| b.twen().set_bit().twint().set_bit().twea().set_bit());
        self.wait_for_send()
    }
    pub fn read_with_nack(self) -> u8 {
        self.twcr.modify(|b| b.twen().set_bit().twint().set_bit());
        self.wait_for_send().twdr.read()
    }
    pub fn read_array<const SIZE: usize>(&self) -> [usize; SIZE] {
        let result = [0; SIZE];
        result
    }
}
