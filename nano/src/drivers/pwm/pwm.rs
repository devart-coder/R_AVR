use crate::drivers::{timers::{action::ActionTrait, RegisterSelection}, Timer};
pub struct Pwm<const N:u8> where () : RegisterSelection<N>{ 
    timer : &'static mut Timer<N>,
}
impl <const N:u8> Pwm<N> where () : RegisterSelection<N>{
	pub fn new(t:&'static mut Timer<N>)->Self{
		let tc =t;
		Self{ timer:tc }
	}
}
pub trait PwmTrait{
    type PwmType;
	fn set_duty_a(&mut self, value:Self::PwmType);
	fn set_duty_b(&mut self, value:Self::PwmType);
    fn set_duty_a_by_per(&mut self, value:u8);
    fn set_duty_b_by_per(&mut self, value:u8);
}
impl PwmTrait for Pwm<0>{
    type PwmType = u8;
    fn set_duty_a(&mut self, value:Self::PwmType){
        self.timer.action().set_counter_a(value);
    }
    fn set_duty_b(&mut self, value:Self::PwmType){
        self.timer.action().set_counter_b(value);
    }
    fn set_duty_a_by_per(&mut self, value:u8){
        self.timer.action().set_counter_a(u8::MAX*(value/100));
    }
    fn set_duty_b_by_per(&mut self, value:u8){
        self.timer.action().set_counter_b(u8::MAX*(value/100));
    }
}