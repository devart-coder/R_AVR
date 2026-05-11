use crate::drivers::{timers::{action::ActionTrait, RegisterSelection}, Timer};
use crate::drivers::timers::settings::*;
use crate::mcu::registers::*;

pub enum PwmMode{
    Normal,
    NonInverted,
    Inverted
}
pub struct Pwm<'a,const N:u8> where () : RegisterSelection<N>{ 
    timer : &'a mut Timer<N>,
}
impl <'a,const N:u8> Pwm<'a,N> where () : RegisterSelection<N>{
	pub fn new(t:&'a mut Timer<N>)->Self{
		Self{ timer:t }
	}
}
//---
pub trait Pwmable<'a,const N:u8> where () : RegisterSelection<N>{
    type PwmType;
    fn a_channel(&'a mut self)->BranchA<'a,N>;
    fn b_channel(&'a mut self)->BranchB<'a,N>;
    fn prescaling(&mut self, p:Prescaling);
}
impl<'a> Pwmable<'a,0> for Pwm<'a,0>{
    type PwmType = u8;
    fn a_channel(&'a mut self)->BranchA<'a,0> {
        BranchA::new(self.timer)
    }
    fn b_channel(&'a mut self)->BranchB<'a,0> {
        BranchB::new(self.timer)
    }
    fn prescaling(&mut self, p:Prescaling){
        self.timer.settings().prescaling(p);
    }
}
//---
pub struct BranchA<'a,const N:u8> where () : RegisterSelection<N>{
    timer : &'a mut Timer<N>,
}
impl<'a> BranchA<'a,0> where () : RegisterSelection<0>{
    pub fn new(t:&'a mut Timer<0>)->BranchA<'a,0>{
        Self{ timer : t }
    }
}
//---
pub struct BranchB<'a,const N:u8> where () : RegisterSelection<N>{
    timer : &'a mut Timer<N>,
}
impl<'a> BranchB<'a,0> where () : RegisterSelection<0>{
    pub fn new(t:&'a mut Timer<0>)->BranchB<'a,0>{
        Self{ timer : t }
    }
}
//---
pub trait Branchable{
    type Type;
    fn set_duty(&mut self, value:Self::Type);
    fn set_mode(&mut self, mode:PwmMode);
}
impl<'a> Branchable for BranchA<'a,0>{
    type Type = u8;
    fn set_duty(&mut self, value:Self::Type){
        self.timer.action().set_counter_a(value);
    }
    fn set_mode(&mut self, mode:PwmMode){
        let bits= match mode {
            PwmMode::Normal =>!((1<<Tccr0aBits::COM0A0 as u8)|(1<<Tccr0aBits::COM0A1 as u8)),
            PwmMode::NonInverted =>!(1<<Tccr0aBits::COM0A0 as u8)&(1<<Tccr0aBits::COM0A1 as u8),
            PwmMode::Inverted =>(1<<Tccr0aBits::COM0A0 as u8)|(1<<Tccr0aBits::COM0A1 as u8),
        };
        if let Some(PwmMode::Normal) = Some(&mode){
            self.timer.settings().tccra().modify(|reg| reg & bits);
        }else{
            self.timer.settings().tccra().modify(|reg| reg | bits);
        }
    }
}
impl<'a> Branchable for BranchB<'a,0>{
    type Type = u8;
    fn set_duty(&mut self, value:Self::Type){
        self.timer.action().set_counter_b(value);
    }
    fn set_mode(&mut self, mode:PwmMode){
        let bits= match mode {
            PwmMode::Normal =>!((1<<Tccr0aBits::COM0B0 as u8)|(1<<Tccr0aBits::COM0B1 as u8)),
            PwmMode::NonInverted =>!(1<<Tccr0aBits::COM0B0 as u8)&(1<<Tccr0aBits::COM0B1 as u8),
            PwmMode::Inverted =>(1<<Tccr0aBits::COM0B0 as u8)|(1<<Tccr0aBits::COM0B1 as u8),
        };
        if let Some(PwmMode::Normal) = Some(&mode){
            self.timer.settings().tccra().modify(|reg| reg & bits);
        }else{
            self.timer.settings().tccra().modify(|reg| reg | bits);
        }
    }
}