use crate::utils::bit_operations::BitOperations;

#[repr(u8)]
pub enum Tccr2aBits{
    Wgm20  = 0,
    Wgm21  = 1,
    Com2b0 = 4,
    Com2b1 = 5,
    Com2a0 = 6,
    Com2a1 = 7,
}
impl Tccr2aBits{
    const WGM20 :u8 = Self::Wgm20 as u8;
    const WGM21 :u8 = Self::Wgm21 as u8;
    const COM2B0:u8 = Self::Com2b0 as u8;
    const COM2B1:u8 = Self::Com2b1 as u8;
    const COM2A0:u8 = Self::Com2a0 as u8;
    const COM2A1:u8 = Self::Com2a1 as u8;
}
pub struct Tccr2aBuilder (pub u8);
impl Tccr2aBuilder{
    pub fn wgm20(self)->BitOperations<{Tccr2aBits::WGM20},Self>{
        BitOperations::<{Tccr2aBits::WGM20},Self>(self)
    }
    pub fn wgm21(self)->BitOperations<{Tccr2aBits::WGM21},Self>{
        BitOperations::<{Tccr2aBits::WGM21},Self>(self)
    }
    pub fn com2b0(self)->BitOperations<{Tccr2aBits::COM2B0},Self>{
        BitOperations::<{Tccr2aBits::COM2B0},Self>(self)
    }
    pub fn com2b1(self)->BitOperations<{Tccr2aBits::COM2B1},Self>{
        BitOperations::<{Tccr2aBits::COM2B1},Self>(self)
    }
    pub fn com2a0(self)->BitOperations<{Tccr2aBits::COM2A0},Self>{
        BitOperations::<{Tccr2aBits::COM2A0},Self>(self)
    }
    pub fn com2a1(self)->BitOperations<{Tccr2aBits::COM2A1},Self>{
        BitOperations::<{Tccr2aBits::COM2A1},Self>(self)
    }
}