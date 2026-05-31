use crate::utils::bit_operations::BitOperations;

pub enum Timsk2Bits{
    Toie2  = 0,
    Ocie2a = 1,
    Ocie2b = 2,
}
impl Timsk2Bits{
    const TOIE2:u8  = Timsk2Bits::Toie2  as u8;
    const OCIE2A:u8 = Timsk2Bits::Ocie2a as u8;
    const OCIE2B:u8 = Timsk2Bits::Ocie2b as u8;
}
pub struct Timsk2Builder (pub u8);
impl Timsk2Builder{
    pub fn toie2(self)->BitOperations<{Timsk2Bits::TOIE2},Self>{
        BitOperations::<{Timsk2Bits::TOIE2},Self>(self)
    }
    pub fn ocie2a(self)->BitOperations<{Timsk2Bits::OCIE2A},Self>{
        BitOperations::<{Timsk2Bits::OCIE2A},Self>(self)
    }
    pub fn ocie2b(self)->BitOperations<{Timsk2Bits::OCIE2B},Self>{
        BitOperations::<{Timsk2Bits::OCIE2B},Self>(self)
    }
    pub fn build(self)->u8{
        self.0
    }
}
