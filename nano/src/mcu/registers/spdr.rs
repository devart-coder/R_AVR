use crate::utils::bit_operations::BitOperations;

//SPI_DataRegister
pub enum SpdrBits{
    Lsb=0,
    Msb=7,
}
impl SpdrBits{
    const LSB:u8=Self::Lsb as u8;
    const MSB:u8=Self::Msb as u8;
}

pub struct SpdrBuilder(pub u8);
impl SpdrBuilder {
    pub fn lsb(self)->BitOperations<{SpdrBits::LSB},Self>{
        BitOperations::<{SpdrBits::LSB},Self>(self)
    }
    pub fn msb(self)->BitOperations<{SpdrBits::MSB},Self>{
        BitOperations::<{SpdrBits::MSB},Self>(self)
    }
}