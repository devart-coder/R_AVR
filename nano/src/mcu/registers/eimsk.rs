use crate::utils::bit_operations::BitOperations;
pub enum EimskBits{
    Int0 = 0,
    Int1 = 1,
}
impl EimskBits{
    pub const INT0:u8 = EimskBits::Int0 as u8;
    pub const INT1:u8 = EimskBits::Int1 as u8;
}
pub struct EimskBuilder (pub u8);
impl EimskBuilder{
    pub fn int0(self)->BitOperations<{EimskBits::INT0},EimskBuilder>{
        BitOperations::<{EimskBits::INT0},EimskBuilder>(self)
    }    
    pub fn int1(self)->BitOperations<{EimskBits::INT1},EimskBuilder>{
        BitOperations::<{EimskBits::INT1},EimskBuilder>(self)
    }
}