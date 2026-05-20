use crate::utils::bit_operations::BitOperations;

#[repr(u8)]
pub enum Tccr0aBits{
    Wgm00  = 0,
    Wgm01  = 1,
    Com0b0 = 4,
    Com0b1 = 5,
    Com0a0 = 6,
    Com0a1 = 7,
}
impl Tccr0aBits{
    const WGM00 :u8 = Self::Wgm00 as u8;
    const WGM01 :u8 = Self::Wgm01 as u8;
    const COM0B0:u8 = Self::Com0b0 as u8;
    const COM0B1:u8 = Self::Com0b1 as u8;
    const COM0A0:u8 = Self::Com0a0 as u8;
    const COM0A1:u8 = Self::Com0a1 as u8;
}
pub struct Tccr0aBuilder (pub u8);
impl Tccr0aBuilder{
    pub fn wgm00(self)->BitOperations<{Tccr0aBits::WGM00},Self>{
        BitOperations::<{Tccr0aBits::WGM00},Self>(self)
    }
    pub fn wgm01(self)->BitOperations<{Tccr0aBits::WGM01},Self>{
        BitOperations::<{Tccr0aBits::WGM01},Self>(self)
    }
    pub fn com0b0(self)->BitOperations<{Tccr0aBits::COM0B0},Self>{
        BitOperations::<{Tccr0aBits::COM0B0},Self>(self)
    }
    pub fn com0b1(self)->BitOperations<{Tccr0aBits::COM0B1},Self>{
        BitOperations::<{Tccr0aBits::COM0B1},Self>(self)
    }
    pub fn com0a0(self)->BitOperations<{Tccr0aBits::COM0A0},Self>{
        BitOperations::<{Tccr0aBits::COM0A0},Self>(self)
    }
    pub fn com0a1(self)->BitOperations<{Tccr0aBits::COM0A1},Self>{
        BitOperations::<{Tccr0aBits::COM0A1},Self>(self)
    }
    pub fn build(self)->u8{
        self.0
    }
}