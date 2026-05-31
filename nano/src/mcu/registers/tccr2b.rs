use crate::utils::bit_operations::BitOperations;

#[repr(u8)]
pub enum Tccr2bBits{
    Cs20=0,
    Cs21=1,
    Cs22=2,
    Wgm22=3,
    Foc2b=6,
    Foc2a=7,
}
impl Tccr2bBits{
    const CS20:u8  = Self::Cs20 as u8;
    const CS21:u8  = Self::Cs21 as u8;
    const CS22:u8  = Self::Cs22 as u8;
    const WGM22:u8 = Self::Wgm22 as u8;
    const FOC2B:u8 = Self::Foc2b as u8;
    const FOC2A:u8 = Self::Foc2a as u8;
}
pub struct Tccr2bBuilder (pub u8);
impl Tccr2bBuilder{
    pub fn cs20(self)->BitOperations<{Tccr2bBits::CS20},Self>{
        BitOperations::<{Tccr2bBits::CS20},Self>(self)
    }
    pub fn cs21(self)->BitOperations<{Tccr2bBits::CS21},Self>{
        BitOperations::<{Tccr2bBits::CS21},Self>(self)
    }
    pub fn cs22(self)->BitOperations<{Tccr2bBits::CS22},Self>{
        BitOperations::<{Tccr2bBits::CS22},Self>(self)
    }
    pub fn wgm22(self)->BitOperations<{Tccr2bBits::WGM22},Self>{
        BitOperations::<{Tccr2bBits::WGM22},Self>(self)    
    }
    pub fn foc2a(self)->BitOperations<{Tccr2bBits::FOC2A},Self>{
        BitOperations::<{Tccr2bBits::FOC2A},Self>(self)    
    }
    pub fn foc2b(self)->BitOperations<{Tccr2bBits::FOC2B},Self>{
        BitOperations::<{Tccr2bBits::FOC2B},Self>(self)  
    }
    pub fn build(self)->u8{
        self.0
    }
}