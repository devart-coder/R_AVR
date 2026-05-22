use crate::utils::bit_operations::BitOperations;

pub enum Timsk0Bits{
    Toie0 = 0,
    Ocie0a = 1,
    Ocie0b = 2,
}
impl Timsk0Bits{
    const TOIE0:u8 = Timsk0Bits::Toie0 as u8;
    const OCIE0A:u8 = Timsk0Bits::Ocie0a as u8;
    const OCIE0B:u8 = Timsk0Bits::Ocie0b as u8;
}
pub struct Timsk0Builder (pub u8);
impl Timsk0Builder{
    pub fn toie0(self)->BitOperations<{Timsk0Bits::TOIE0},Self>{
        BitOperations::<{Timsk0Bits::TOIE0},Self>(self)
    }
    pub fn ocie0a(self)->BitOperations<{Timsk0Bits::OCIE0A},Self>{
        BitOperations::<{Timsk0Bits::OCIE0A},Self>(self)
    }
    pub fn ocie0b(self)->BitOperations<{Timsk0Bits::OCIE0B},Self>{
        BitOperations::<{Timsk0Bits::OCIE0B},Self>(self)
    }
}