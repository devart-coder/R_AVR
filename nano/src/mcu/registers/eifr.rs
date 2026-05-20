use crate::utils::bit_operations::BitOperations;

pub enum EifrBits{
    Intf0 = 0,
    Intf1 = 1,
}
impl EifrBits{
    pub const INTF0:u8 = EifrBits::Intf0 as u8;
    pub const INTF1:u8 = EifrBits::Intf1 as u8;
}
pub struct EifrBuilder(pub u8);
impl EifrBuilder {
    pub fn intf0(self)->BitOperations<{EifrBits::INTF0},Self>{
        BitOperations::<{EifrBits::INTF0},Self> (self)
    }
    pub fn intf1(self)->BitOperations<{EifrBits::INTF1},Self>{
        BitOperations::<{EifrBits::INTF1},Self> (self)
    }
}