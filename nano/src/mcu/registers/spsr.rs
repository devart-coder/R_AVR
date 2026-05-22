use crate::utils::bit_operations::BitOperations;

//SpiStatusRegister
pub enum SpsrBits{
    Spix2 = 0,//SpiDoubleSpeedBit
    Wcol  = 6,//WriteCollisionFlag
    Spif  = 7,//SpiInterruptFlag
}
impl SpsrBits{
    const SPIX2:u8 = Self::Spix2 as u8;
    const WCOL :u8 = Self::Wcol  as u8;
    const SPIF :u8 = Self::Spif  as u8;
}

pub struct SpsrBuilder (pub u8);
impl SpsrBuilder{
    pub fn spix2(self)->BitOperations<{SpsrBits::SPIX2},Self>{
        BitOperations::<{SpsrBits::SPIX2},Self>(self)
    }
    pub fn wcol(self)->BitOperations<{SpsrBits::WCOL},Self>{
        BitOperations::<{SpsrBits::WCOL},Self>(self)
    }
    pub fn spif(self)->BitOperations<{SpsrBits::SPIF},Self>{
        BitOperations::<{SpsrBits::SPIF},Self>(self)
    }
}