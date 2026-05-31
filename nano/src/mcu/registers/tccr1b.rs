use crate::utils::bit_operations::BitOperations;

#[repr(u8)]
pub enum Tccr1bBits{
    Cs10=0,
    Cs11=1,
    Cs12=2,
    Wgm12=3,
    Wgm13=4,
    Ices1=6,
    Icnc1=7,
}
impl Tccr1bBits{
    const CS10 :u8  = Self::Cs10 as u8;
    const CS11 :u8  = Self::Cs11 as u8;
    const CS12 :u8  = Self::Cs12 as u8;
    const WGM12:u8 = Self::Wgm12 as u8;
    const WGM13:u8 = Self::Wgm13 as u8;
    const ICES1:u8 = Self::Ices1 as u8;
    const ICNC1:u8 = Self::Icnc1 as u8;
}
pub struct Tccr1bBuilder (pub u8);
impl Tccr1bBuilder{
    pub fn cs10(self)->BitOperations<{Tccr1bBits::CS10},Self>{
        BitOperations::<{Tccr1bBits::CS10},Self>(self)
    }
    pub fn cs11(self)->BitOperations<{Tccr1bBits::CS11},Self>{
        BitOperations::<{Tccr1bBits::CS11},Self>(self)
    }
    pub fn cs12(self)->BitOperations<{Tccr1bBits::CS12},Self>{
        BitOperations::<{Tccr1bBits::CS12},Self>(self)
    }
    pub fn wgm12(self)->BitOperations<{Tccr1bBits::WGM12},Self>{
        BitOperations::<{Tccr1bBits::WGM12},Self>(self)    
    }
    pub fn wgm13(self)->BitOperations<{Tccr1bBits::WGM13},Self>{
        BitOperations::<{Tccr1bBits::WGM13},Self>(self)    
    }
    pub fn ices1(self)->BitOperations<{Tccr1bBits::ICES1},Self>{
        BitOperations::<{Tccr1bBits::ICES1},Self>(self)    
    }
    pub fn icnc1(self)->BitOperations<{Tccr1bBits::ICNC1},Self>{
        BitOperations::<{Tccr1bBits::ICNC1},Self>(self)  
    }
    pub fn build(self)->u8{
        self.0
    }

}