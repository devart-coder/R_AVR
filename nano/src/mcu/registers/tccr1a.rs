use crate::utils::bit_operations::BitOperations;

#[repr(u8)]
pub enum Tccr1aBits{
    Wgm10  = 0,
    Wgm11  = 1,
    Com1b0 = 4,
    Com1b1 = 5,
    Com1a0 = 6,
    Com1a1 = 7,
}
impl Tccr1aBits {
    const WGM10 :u8 = Self::Wgm10 as u8;
    const WGM11 :u8 = Self::Wgm11 as u8;
    const COM1B0:u8 = Self::Com1b0 as u8;
    const COM1B1:u8 = Self::Com1b1 as u8;
    const COM1A0:u8 = Self::Com1a0 as u8;
    const COM1A1:u8 = Self::Com1a1 as u8;
}
pub struct Tccr1aBuilder (pub u8);
impl Tccr1aBuilder{
    pub fn wgm10(self)->BitOperations<{Tccr1aBits::WGM10},Self>{
        BitOperations::<{Tccr1aBits::WGM10},Self>(self)
    }
    pub fn wgm11(self)->BitOperations<{Tccr1aBits::WGM11},Self>{
        BitOperations::<{Tccr1aBits::WGM11},Self>(self)
    }
    pub fn com1b0(self)->BitOperations<{Tccr1aBits::COM1B0},Self>{
        BitOperations::<{Tccr1aBits::COM1B0},Self>(self)
    }
    pub fn com1b1(self)->BitOperations<{Tccr1aBits::COM1B1},Self>{
        BitOperations::<{Tccr1aBits::COM1B1},Self>(self)
    }
    pub fn com1a0(self)->BitOperations<{Tccr1aBits::COM1A0},Self>{
        BitOperations::<{Tccr1aBits::COM1A0},Self>(self)
    }
    pub fn com1a1(self)->BitOperations<{Tccr1aBits::COM1A1},Self>{
        BitOperations::<{Tccr1aBits::COM1A1},Self>(self)
    }
}