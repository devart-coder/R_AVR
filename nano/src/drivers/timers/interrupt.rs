use crate::mcu::registers::timsk1::Timsk1Builder;
use crate::mcu::registers::timsk2::Timsk2Builder;
use crate::{mcu::registers_list::*, utils::register::Register};
use crate::mcu::registers::timsk0::*;
pub struct Interrupt <const N:u8>{
    timsk : Register,
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
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk0Builder(0)
		    			.ocie0a()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk0Builder(0)
		    			.ocie0a()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
	fn b_channel_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk0Builder(0)
		    			.ocie0b()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk0Builder(0)
		    			.ocie0b()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
	fn overflow_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk0Builder(0)
		    			.toie0()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk0Builder(0)
		    			.toie0()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
}

impl InterruptTrait for Interrupt<1>{
    fn a_channel_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk1Builder(0)
		    			.ocie1a()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk1Builder(0)
		    			.ocie1a()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
	fn b_channel_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk1Builder(0)
		    			.ocie1b()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk1Builder(0)
		    			.ocie1b()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
	fn overflow_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk1Builder(0)
		    			.toie1()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk1Builder(0)
		    			.toie1()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
}

impl InterruptTrait for Interrupt<2>{
    fn a_channel_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk2Builder(0)
		    			.ocie2a()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk2Builder(0)
		    			.ocie2a()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
	fn b_channel_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk2Builder(0)
		    			.ocie2b()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk2Builder(0)
		    			.ocie2b()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
	fn overflow_enable(&mut self, value:bool){
	    match value{
		    true => 
		    		self.timsk.modify(|w| w | 
		    			Timsk2Builder(0)
		    			.toie2()
		    			.set_bit()
		    			.build()
		    		)
	    		,
			false =>
				self.timsk.modify(|w| w&
					Timsk2Builder(0)
		    			.toie2()
		    			.clear_bit()
		    			.build()
		    	),
		}
	}
}
