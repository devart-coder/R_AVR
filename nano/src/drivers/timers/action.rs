use crate::mcu::registers::*;
use crate::utils::register::*;
use crate::drivers::timers::RegisterSelection;
pub struct Action<const N:u8>where ():RegisterSelection<N> {
    ocra:<() as RegisterSelection<N>>::RegType,
    ocrb:<() as RegisterSelection<N>>::RegType,
    tcnt:<() as RegisterSelection<N>>::RegType,
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

pub trait ActionTrait{
    type ArgType;
    fn set_counter_a(&mut self, value:Self::ArgType);
    fn set_counter_b(&mut self, value:Self::ArgType);
    fn set_timer_counter(&mut self, value:Self::ArgType);
    fn counter_a(&self)->Self::ArgType;
    fn counter_b(&self)->Self::ArgType;
    fn timer_counter(&self)->Self::ArgType;
}
impl ActionTrait for Action<0>{
    type ArgType = u8;
    fn set_counter_a(&mut self, value:Self::ArgType){
        self.ocra.write(value);
    }
    fn set_counter_b(&mut self, value:Self::ArgType){
        self.ocrb.write(value);
    }
    fn set_timer_counter(&mut self, value:Self::ArgType){
        self.tcnt.write(value);
    }
    fn counter_a(&self)->Self::ArgType{
        self.ocra.read()
    }
    fn counter_b(&self)->Self::ArgType{
        self.ocrb.read()
    }
    fn timer_counter(&self)->Self::ArgType{
        self.tcnt.read()
    }
}
impl ActionTrait for Action<1>{
    type ArgType = u16;
    fn set_counter_a(&mut self, value:Self::ArgType){
        self.ocra.write(value);
    }
    fn set_counter_b(&mut self, value:Self::ArgType){
        self.ocrb.write(value);
    }
    fn set_timer_counter(&mut self, value:Self::ArgType){
    }
    fn counter_a(&self)->Self::ArgType{
        self.ocra.read()
    }
    fn counter_b(&self)->Self::ArgType{
        self.ocrb.read()
    }
    fn timer_counter(&self)->Self::ArgType{
        self.tcnt.read()
    }
}
