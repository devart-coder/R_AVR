use super::action_trait::*;
use crate::mcu::registers::*;
use crate::utils::register::*;

pub struct Action<const N:u8>where ():RegisterSelection<N> {
    pub ocra:<() as RegisterSelection<N>>::RegType,
    pub ocrb:<() as RegisterSelection<N>>::RegType,
    pub tcnt:<() as RegisterSelection<N>>::RegType,
}
impl Action<0>{
    pub const fn new()->Self{
        Self{
            ocra: Register::new(TimerRegisters::OCR0A as u8),
            ocrb: Register::new(TimerRegisters::OCR0B as u8),
            tcnt: Register::new(TimerRegisters::TCNT0 as u8),
        }
    }
}
impl Action<2>{
    pub const fn new()->Self{
        Self{
            ocra: Register::new(TimerRegisters::OCR2A as u8),
            ocrb: Register::new(TimerRegisters::OCR2B as u8),
            tcnt: Register::new(TimerRegisters::TCNT2 as u8),
        }
    }
}
impl Action<1>{
    pub const fn new()->Self{
        Self{
            ocra: RegisterU16Builder::builder()
                .set_lreg(TimerRegisters::OCR1AL as u8)
                .set_hreg(TimerRegisters::OCR1AH as u8)
                .build(),
            ocrb: RegisterU16Builder::builder()
                .set_lreg(TimerRegisters::OCR1BL as u8)
                .set_hreg(TimerRegisters::OCR1BH as u8)
                .build(),
            tcnt: RegisterU16Builder::builder()
                .set_lreg(TimerRegisters::TCNT1L as u8)
                .set_hreg(TimerRegisters::TCNT1H as u8)
                .build(),
        }
    }
}
