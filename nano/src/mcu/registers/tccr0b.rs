use crate::utils::bit_operations::BitOperations;

#[repr(u8)]
pub enum Tccr0bBits{
    Cs00  = 0,
    Cs01  = 1,
    Cs02  = 2,
    Wgm02 = 3,
    Foc0b = 6,
    Foc0a = 7
}
impl Tccr0bBits{
    const CS00:u8  = Self::Cs00 as u8;
    const CS01:u8  = Self::Cs01 as u8;
    const CS02:u8  = Self::Cs02 as u8;
    const WGM02:u8 = Self::Wgm02 as u8;
    const FOC0B:u8 = Self::Foc0b as u8;
    const FOC0A:u8 = Self::Foc0a as u8;
}
pub struct Tccr0bBuilder (pub u8);
impl Tccr0bBuilder{
    pub fn cs00(self)->BitOperations<{Tccr0bBits::CS00},Self>{
        BitOperations::<{Tccr0bBits::CS00},Self>(self)
    }
    pub fn cs01(self)->BitOperations<{Tccr0bBits::CS01},Self>{
        BitOperations::<{Tccr0bBits::CS01},Self>(self)
    }
    pub fn cs02(self)->BitOperations<{Tccr0bBits::CS02},Self>{
        BitOperations::<{Tccr0bBits::CS02},Self>(self)
    }
    pub fn wgm02(self)->BitOperations<{Tccr0bBits::WGM02},Self>{
        BitOperations::<{Tccr0bBits::WGM02},Self>(self)    
    }
    pub fn foc0a(self)->BitOperations<{Tccr0bBits::FOC0A},Self>{
        BitOperations::<{Tccr0bBits::FOC0A},Self>(self)    
    }
    pub fn foc0b(self)->BitOperations<{Tccr0bBits::FOC0B},Self>{
        BitOperations::<{Tccr0bBits::FOC0B},Self>(self)  
    }
    pub fn build(self)->u8{
        self.0
    }
}