use crate::utils::bit_operations::BitOperations;

pub enum TwbrBits{
    Twbr0 = 0,
    Twbr1 = 1,
    Twbr2 = 2,
    Twbr3 = 3,
    Twbr4 = 4,
    Twbr5 = 5,
    Twbr6 = 6,
    Twbr7 = 7,
}

impl TwbrBits {
    const TWBR0:u8 = Self::Twbr0 as u8;
    const TWBR1:u8 = Self::Twbr1 as u8;
    const TWBR2:u8 = Self::Twbr2 as u8;
    const TWBR3:u8 = Self::Twbr3 as u8;
    const TWBR4:u8 = Self::Twbr4 as u8;
    const TWBR5:u8 = Self::Twbr5 as u8;
    const TWBR6:u8 = Self::Twbr6 as u8;
    const TWBR7:u8 = Self::Twbr7 as u8;
}

pub struct TwbrBuilder (pub u8);
impl TwbrBuilder{
    pub fn twbr0(self)->BitOperations::<{TwbrBits::TWBR0},Self>{
        BitOperations::<{TwbrBits::TWBR0},Self>(self)
    }
    pub fn twbr1(self)->BitOperations::<{TwbrBits::TWBR1},Self>{
        BitOperations::<{TwbrBits::TWBR1},Self>(self)
    }
    pub fn twbr2(self)->BitOperations::<{TwbrBits::TWBR2},Self>{
        BitOperations::<{TwbrBits::TWBR2},Self>(self)
    }
    pub fn twbr3(self)->BitOperations::<{TwbrBits::TWBR3},Self>{
        BitOperations::<{TwbrBits::TWBR3},Self>(self)
    }
    pub fn twbr4(self)->BitOperations::<{TwbrBits::TWBR4},Self>{
        BitOperations::<{TwbrBits::TWBR4},Self>(self)
    }
    pub fn twbr5(self)->BitOperations::<{TwbrBits::TWBR5},Self>{
        BitOperations::<{TwbrBits::TWBR5},Self>(self)
    }
    pub fn twbr6(self)->BitOperations::<{TwbrBits::TWBR6},Self>{
        BitOperations::<{TwbrBits::TWBR6},Self>(self)
    }
    pub fn twbr7(self)->BitOperations::<{TwbrBits::TWBR7},Self>{
        BitOperations::<{TwbrBits::TWBR7},Self>(self)
    }
    pub fn build(self)->u8{
        self.0
    }
}