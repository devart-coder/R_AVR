use crate::utils::bit_operations::BitOperations;

pub enum EicraBits{
    Isc00 = 0,
    Isc01 = 1,
    Isc10 = 2,
    Isc11 = 3,
}
impl EicraBits{
    pub const ISC00:u8 = EicraBits::Isc00 as u8;
    pub const ISC01:u8 = EicraBits::Isc01 as u8;
    pub const ISC10:u8 = EicraBits::Isc10 as u8;
    pub const ISC11:u8 = EicraBits::Isc11 as u8;
}
//---
pub struct EicraBuilder(pub u8);
impl EicraBuilder{
    pub fn isc00(self)->BitOperations::<{EicraBits::ISC00},EicraBuilder>{
        BitOperations::<{EicraBits::ISC00},EicraBuilder>(self)
    }
    pub fn isc01(self)->BitOperations<{EicraBits::ISC01},EicraBuilder>{
        BitOperations::<{EicraBits::ISC01},EicraBuilder>(self)
    }
    pub fn isc10(self)->BitOperations<{EicraBits::ISC10},EicraBuilder>{
        BitOperations::<{EicraBits::ISC10},EicraBuilder>(self)
    }
    pub fn isc11(self)->BitOperations<{EicraBits::ISC11},EicraBuilder>{
        BitOperations::<{EicraBits::ISC11},EicraBuilder>(self)
    }
}