use core::{borrow::Borrow, ops::Deref};

pub enum ExIntRegisters{
    EICRA = 0x69,
    EIFR = 0x1C,
    EIMSK = 0x1D,
}
    


pub enum EifrBits{
    INTF0 = 0,
    INTF1 = 1,
}
impl EifrBits{
    pub const Intf0:u8 = EifrBits::INTF0 as u8;
    pub const Intf1:u8 = EifrBits::INTF1 as u8;
}

pub trait Bitsable{
    fn set_mask(&mut self, mask:u8);
    fn reset_mask(&mut self, mask:u8);
    fn toggle_mask(&mut self, mask:u8);
}
macro_rules! impl_bits_able {
        ($($t:ty),*) => {
        $(
            impl Bitsable for $t {
                fn set_mask(&mut self, mask: u8) { self.0 |= mask; }
                fn reset_mask(&mut self, mask: u8) { self.0 &= !mask; }
                fn toggle_mask(&mut self, mask: u8) { self.0 ^= mask; }
            }
        )*
    };
}
impl_bits_able!(EicraBuilder,EimskBuilder);

pub struct BitsOperation<const BIT:u8,T:Bitsable> (T);
impl<const BIT:u8, T:Bitsable> BitsOperation<BIT, T>{
    const MASK:u8 = 1<<BIT;
    pub fn set_bit(mut self)->T{
        self.0.set_mask( Self::MASK);
        self.0
    }
    pub fn clear_bit(mut self)->T{
        self.0.reset_mask(Self::MASK);
        self.0
    }
    pub fn toogle(mut self) -> T{
        self.0.toggle_mask(Self::MASK);
        self.0
    }
}

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
pub struct EicraBuilder(pub u8);
impl EicraBuilder{
    pub fn isc00(self)->BitsOperation::<{EicraBits::ISC00},EicraBuilder>{
        BitsOperation::<{EicraBits::ISC00},EicraBuilder>(self)
    }
    pub fn isc01(self)->BitsOperation<{EicraBits::ISC01},EicraBuilder>{
        BitsOperation::<{EicraBits::ISC01},EicraBuilder>(self)
    }
    pub fn isc10(self)->BitsOperation<{EicraBits::ISC10},EicraBuilder>{
        BitsOperation::<{EicraBits::ISC10},EicraBuilder>(self)
    }
    pub fn isc11(self)->BitsOperation<{EicraBits::ISC11},EicraBuilder>{
        BitsOperation::<{EicraBits::ISC11},EicraBuilder>(self)
    }
}    

pub enum EimskBits{
    Int0 = 0,
    Int1 = 1,
}
impl EimskBits{
    pub const INT0:u8 = EimskBits::Int0 as u8;
    pub const INT1:u8 = EimskBits::Int1 as u8;
}
pub struct EimskBuilder (u8);
impl EimskBuilder{
    pub fn int0(self)->BitsOperation<{EimskBits::INT0},EimskBuilder>{
        BitsOperation::<{EimskBits::INT0},EimskBuilder>(self)
    }    
    pub fn int1(self)->BitsOperation<{EimskBits::INT1},EimskBuilder>{
        BitsOperation::<{EimskBits::INT1},EimskBuilder>(self)
    }
}