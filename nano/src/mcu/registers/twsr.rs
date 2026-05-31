use crate::utils::{bit_operations::BitOperations, register::Register};

pub struct Twsr {
    reg: Register,
}
impl Twsr {
    const ADDRESS: u8 = 0xB9;
    pub fn new() -> Self {
        Self {
            reg: Register::new(Self::ADDRESS),
        }
    }
    pub fn read(&self) -> u8 {
        self.reg.read()
    }
    pub fn modify<F>(&self, builder: F)
    where
        F: FnOnce(TwsrBuilder) -> TwsrBuilder,
    {
        self.reg.modify(|reg| reg | builder(TwsrBuilder(0)).build());
    }
    pub fn write(&self, value: u8) {
        self.reg.write(value);
    }
}

pub enum TwsrBits {
    //TWIStatusRegister
    Twps0 = 0, //TWI Prescaler 0
    Twps1 = 1, //TWI Prescaler 1
    Tws3 = 3,  //TWI Status
    Tws4 = 4,  //TWI Status
    Tws5 = 5,  //TWI Status
    Tws6 = 6,  //TWI Status
    Tws7 = 7,  //TWI Status
}
impl TwsrBits {
    const TWPS0: u8 = Self::Twps0 as u8;
    const TWPS1: u8 = Self::Twps1 as u8;
    const TWS3: u8 = Self::Tws3 as u8;
    const TWS4: u8 = Self::Tws4 as u8;
    const TWS5: u8 = Self::Tws5 as u8;
    const TWS6: u8 = Self::Tws6 as u8;
    const TWS7: u8 = Self::Tws7 as u8;
}

pub struct TwsrBuilder(pub u8);
impl TwsrBuilder {
    pub fn twps0(self) -> BitOperations<{ TwsrBits::TWPS0 }, Self> {
        BitOperations::<{ TwsrBits::TWPS0 }, Self>(self)
    }
    pub fn twps1(self) -> BitOperations<{ TwsrBits::TWPS1 }, Self> {
        BitOperations::<{ TwsrBits::TWPS1 }, Self>(self)
    }
    pub fn build(self) -> u8 {
        self.0
    }
}
