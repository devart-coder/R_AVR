use crate::utils::bit_operations::BitOperations;

//Cpha bit
//Level - Leading Edge - Trailing Edge
//  0       Sample          Setup
//  1       Setup           Sample
//---
//Spr0,Spr1 bits
//  SPI2X   SPER1   SPR0    Freq
//    0       0       0       f/4
//    0       0       1       f/16
//    0       1       0       f/64
//    0       1       1       f/128
//    1       0       0       f/2
//    1       0       1       f/8
//    1       1       0       f/32
//    1       1       1       f/64
enum SpcrBits{
    Spr0 = 0,//SpiClockRate1
    Spr1 = 1,//SpiClockRate1
    Cpha = 2,//ClockPhase
    Cpol = 3,//ClockPolarity
    Mstr = 4,//Master/SlaveSelect
    Dord = 5,//DataOrder
    Spe  = 6,//SpiEnable
    Spie = 7,//SpiInterruptEnable
}
impl SpcrBits{
    const SPR0:u8 = SpcrBits::Spr0 as u8;
    const SPR1:u8 = SpcrBits::Spr1 as u8;
    const CPHA:u8 = SpcrBits::Cpha as u8;
    const CPOL:u8 = SpcrBits::Cpol as u8;
    const MSTR:u8 = SpcrBits::Mstr as u8;
    const DORD:u8 = SpcrBits::Dord as u8;
    const SPE :u8 = SpcrBits::Spe  as u8;
    const SPIE:u8 = SpcrBits::Spie as u8;
}

pub struct SpcrBuilder(pub u8);
impl SpcrBuilder{
   pub fn spr0(self)->BitOperations::<{SpcrBits::SPR0},Self>{
       BitOperations::<{SpcrBits::SPR0},Self>(self)
   } 
   pub fn spr1(self)->BitOperations::<{SpcrBits::SPR1},Self>{
       BitOperations::<{SpcrBits::SPR1},Self>(self)
   }
   pub fn cpha(self)->BitOperations::<{SpcrBits::CPHA},Self>{
       BitOperations::<{SpcrBits::CPHA},Self>(self)
   }
   pub fn cpol(self)->BitOperations::<{SpcrBits::CPOL},Self>{
       BitOperations::<{SpcrBits::CPOL},Self>(self)
   }
   pub fn mstr(self)->BitOperations::<{SpcrBits::MSTR},Self>{
       BitOperations::<{SpcrBits::MSTR},Self>(self)
   }
   pub fn dodr(self)->BitOperations::<{SpcrBits::DORD},Self>{
       BitOperations::<{SpcrBits::DORD},Self>(self)
   }
   pub fn spe(self)->BitOperations::<{SpcrBits::SPE},Self>{
       BitOperations::<{SpcrBits::SPE},Self>(self)
   }
   pub fn spie(self)->BitOperations::<{SpcrBits::SPIE},Self>{
       BitOperations::<{SpcrBits::SPIE},Self>(self)
   }
   pub fn build(self)->u8{
       self.0
   } 
}
