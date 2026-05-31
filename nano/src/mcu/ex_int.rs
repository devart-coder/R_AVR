
use crate::utils::register::Register;

use super::registers::eicra::*;
//D2 - 0 - Bits::ISC00,ISC01
//D3 - 1 - Bits::ISC10,ISC11

//0,0 - Low,
//0,1 - Change,
//1,0 - Falling,
//1,1 - Rising,
pub enum InterruptMode{
    Low,
    Change,
    Rising,
    Falling 
}
pub enum ExIntRegisters{
    EICRA = 0x69,
    EIFR = 0x1C,
    EIMSK = 0x1D,
}
pub struct ExInt<const N:u8>{
    eimsk : Register,
    eicra : Register,
}
impl ExInt<0>{
    pub fn new()->Self{
        Self{
            eimsk : Register::new(ExIntRegisters::EIMSK as u8),
            eicra : Register::new(ExIntRegisters::EICRA as u8),
        }
    }
    pub fn set_mod(&mut self, mode:InterruptMode){
        let _result = match mode{
            InterruptMode::Low=>0,
            InterruptMode::Change=>0,
            InterruptMode::Rising=>0,
            InterruptMode::Falling=>0,
        };
    }
}
impl ExInt<1>{
    pub fn new()->Self{
        EicraBuilder(0)
        .isc00().set_bit()
        .isc01().set_bit()
        .isc10().toogle();
        Self{
            eimsk : Register::new(ExIntRegisters::EIMSK as u8),
            eicra : Register::new(ExIntRegisters::EICRA as u8),
        }
    }
}

