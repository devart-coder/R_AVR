use crate::utils::{bit_operations::BitOperations, register::Register};
pub struct Twcr {
    register: Register,
}
impl Twcr {
    const ADDRESS: u8 = 0xBC;
    pub fn new() -> Self {
        Self {
            register: Register::new(Self::ADDRESS),
        }
    }
    pub fn modify<F>(&self, builder: F)
    where
        F: FnOnce(TwcrBuilder) -> TwcrBuilder,
    {
        let result = builder(TwcrBuilder(0)).build();
        self.register.write(result);
    }
    pub fn write(&self, value: u8) {
        self.register.write(value);
    }
    pub fn read(&self) -> u8 {
        self.register.read()
    }
}

pub enum TwcrBits {
    //TwiControlRegister
    Twie = 0,  //TWIInterruptEnable
    Twen = 2,  //TWIEnable
    Twwc = 3,  //TWIWriteCollisionFlag
    Twsto = 4, //TWIStop
    Twsta = 5, //TWIStart
    Twea = 6,  //TWIEnableAcknowledge
    Twin = 7,  //TWIInterrruptFlag
}
impl TwcrBits {
    const TWIE: u8 = Self::Twie as u8;
    const TWEN: u8 = Self::Twen as u8;
    const TWWC: u8 = Self::Twwc as u8;
    const TWSTO: u8 = Self::Twsto as u8;
    const TWSTA: u8 = Self::Twsta as u8;
    const TWEA: u8 = Self::Twea as u8;
    const TWIN: u8 = Self::Twin as u8;
}

pub struct TwcrBuilder(pub u8);
impl TwcrBuilder {
    pub fn twie(self) -> BitOperations<{ TwcrBits::TWIE }, Self> {
        BitOperations::<{ TwcrBits::TWIE }, Self>(self)
    }
    pub fn twen(self) -> BitOperations<{ TwcrBits::TWEN }, Self> {
        BitOperations::<{ TwcrBits::TWEN }, Self>(self)
    }
    pub fn twwc(self) -> BitOperations<{ TwcrBits::TWWC }, Self> {
        BitOperations::<{ TwcrBits::TWWC }, Self>(self)
    }
    pub fn twsto(self) -> BitOperations<{ TwcrBits::TWSTO }, Self> {
        BitOperations::<{ TwcrBits::TWSTO }, Self>(self)
    }
    pub fn twsta(self) -> BitOperations<{ TwcrBits::TWSTA }, Self> {
        BitOperations::<{ TwcrBits::TWSTA }, Self>(self)
    }
    pub fn twea(self) -> BitOperations<{ TwcrBits::TWEA }, Self> {
        BitOperations::<{ TwcrBits::TWEA }, Self>(self)
    }
    pub fn twint(self) -> BitOperations<{ TwcrBits::TWIN }, Self> {
        BitOperations::<{ TwcrBits::TWIN }, Self>(self)
    }
    pub fn build(self) -> u8 {
        self.0
    }
}
