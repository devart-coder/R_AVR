use crate::{mcu::registers::*, utils::register::Register};

pub struct Interrupt <const N:u8>{
    pub timsk : Register,
}
impl Interrupt<0>{
    pub const fn new ()->Self{
	    Self { timsk : Register::new(TimerRegisters::TIMSK0 as u8), }
	}
}
impl Interrupt<1>{
    pub const fn new ()->Self{
	    Self { timsk : Register::new(TimerRegisters::TIMSK1 as u8), }
	}
}
impl Interrupt<2>{
    pub const fn new ()->Self{
	    Self { timsk : Register::new(TimerRegisters::TIMSK2 as u8), }
	}
}

pub trait InterruptTrait{
    fn a_channel_enable(&mut self, value:bool);
	fn b_channel_enable(&mut self, value:bool);
	fn overflow_enable(&mut self, value:bool);
}
impl InterruptTrait for Interrupt<0>{
    fn a_channel_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk0Bits::OCIE0A as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk0Bits::OCIE0A as u8)),
		}
	}
	fn b_channel_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk0Bits::OCIE0B as u8)),
			false=> self.timsk.modify(|w| w& !(1<<Timsk0Bits::OCIE0B as u8)),
		}
	}
	fn overflow_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk0Bits::TOIE0 as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk0Bits::TOIE0 as u8)),
		}
	}
}
impl InterruptTrait for Interrupt<1>{
    fn a_channel_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk1Bits::OCIE1A as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk1Bits::OCIE1A as u8)),
		}
	}
	fn b_channel_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk1Bits::OCIE1B as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk1Bits::OCIE1B as u8)),
		}
	}
	fn overflow_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk1Bits::TOIE1 as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk1Bits::TOIE1 as u8)),
		}
	}
}
impl InterruptTrait for Interrupt<2>{
    fn a_channel_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk2Bits::OCIE2A as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk2Bits::OCIE2A as u8)),
		}
	}
	fn b_channel_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk2Bits::OCIE2B as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk2Bits::OCIE2B as u8)),
		}
	}
	fn overflow_enable(&mut self, value:bool){
	    match value{
		    true => self.timsk.modify(|w| w|(1<<Timsk2Bits::TOIE2 as u8)),
			false=> self.timsk.modify(|w| w&!(1<<Timsk2Bits::TOIE2 as u8)),
		}
	}
}
