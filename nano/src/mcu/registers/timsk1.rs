use crate::utils::bit_operations::BitOperations;

pub enum Timsk1Bits{
    Toie1  = 0,
    Ocie1a = 1,
    Ocie1b = 2,
    Icie1  = 5,
}
impl Timsk1Bits{
    const TOIE1:u8 = Timsk1Bits::Toie1 as u8;
    const OCIE1A:u8 = Timsk1Bits::Ocie1a as u8;
    const OCIE1B:u8 = Timsk1Bits::Ocie1b as u8;
    const ICIE1:u8 = Timsk1Bits::Icie1 as u8;
}
pub struct Timsk1Builder (pub u8);
impl Timsk1Builder{
    pub fn toie1(self)->BitOperations<{Timsk1Bits::TOIE1},Self>{
        BitOperations::<{Timsk1Bits::TOIE1},Self>(self)
    }
    pub fn ocie1a(self)->BitOperations<{Timsk1Bits::OCIE1A},Self>{
        BitOperations::<{Timsk1Bits::OCIE1A},Self>(self)
    }
    pub fn ocie1b(self)->BitOperations<{Timsk1Bits::OCIE1B},Self>{
        BitOperations::<{Timsk1Bits::OCIE1B},Self>(self)
    }
    pub fn icie1(self)->BitOperations<{Timsk1Bits::ICIE1},Self>{
        BitOperations::<{Timsk1Bits::ICIE1},Self>(self)
    }
    pub fn build(self)->u8{
        self.0
    }
}