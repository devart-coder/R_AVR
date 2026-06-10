use crate::{
    drivers::twi::master::Master,
    mcu::registers::{twbr::Twbr, twsr::Twsr},
};
pub struct Twi {
    twsr: Twsr,
    twbr: Twbr,
}
impl Twi {
    pub fn new() -> Self {
        Self {
            twsr: Twsr::new(),
            twbr: Twbr::new(),
        }
    }
    pub fn speed(self, value: u8) -> Self {
        self.twbr.write(value);
        self
    }
    pub fn prescaler_mode(self, mode: PrescalerMode) -> Self {
        match mode {
            PrescalerMode::_1 => self
                .twsr
                .modify(|b| b.twps1().clear_bit().twps0().clear_bit()),
            PrescalerMode::_4 => self
                .twsr
                .modify(|b| b.twps1().clear_bit().twps0().set_bit()),
            PrescalerMode::_16 => self
                .twsr
                .modify(|b| b.twps1().set_bit().twps0().clear_bit()),
            PrescalerMode::_64 => self.twsr.modify(|b| b.twps1().set_bit().twps0().set_bit()),
        }
        self
    }
    pub fn as_master(self) -> Master {
        Master::new()
    }
}
pub enum PrescalerMode {
    _1 = 1,
    _4 = 4,
    _16 = 16,
    _64 = 64,
}
