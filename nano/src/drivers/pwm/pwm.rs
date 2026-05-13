use core::{borrow::Borrow, cell::RefCell, ops::Deref};

use crate::drivers::{timers::{action::{Action, ActionTrait}, RegisterSelection}, Timer};
use crate::drivers::timers::settings::*;
use crate::mcu::registers::*;

pub enum PwmMode{
    Normal,
    NonInverted,
    Inverted
}

pub struct Pwm<'a, const N:u8> where () : RegisterSelection<N>{ 
    timer : &'a mut Timer<N>,
}
impl <'a, const N:u8> Pwm<'a, N> where () : RegisterSelection<N>{
	pub fn new(t:&'a mut Timer<N>)->Self{
		Self{ timer:t }
	}
}

pub trait Pwmable<'a, const N:u8> where () : RegisterSelection<N>{
    type PwmType;
    fn channels(&mut self)->(BranchA<N>,BranchB<N>);
    fn prescaling(&mut self, p:Prescaling);
}
impl<'a> Pwmable<'a,0> for Pwm<'a,0>{ 
    type PwmType = u8;
    fn channels(&mut self)->(BranchA<0>,BranchB<0>){
        let s_a = self.timer.settings().clone();
        let s_b = self.timer.settings().clone();
        let a_a = self.timer.action().clone();
        let a_b = self.timer.action().clone();
        ( BranchA::new(s_a,a_a), BranchB::new(s_b,a_b) )
    }
    fn prescaling(&mut self, p:Prescaling){
        self.timer.settings().prescaling(p);
    }
}
//---
pub struct BranchA<const N:u8> where () : RegisterSelection<N>{
    settings : RefCell<Settings<N>>,
    action   : RefCell<Action<N>>, 
}
impl<'a> BranchA<0> where () : RegisterSelection<0>{
    pub fn new(s: Settings<0>, a: Action<0>)->BranchA<0>{
        Self { settings : RefCell::new(s), action : RefCell::new(a) }
    }
}
pub struct BranchB<const N:u8> where () : RegisterSelection<N>{
    settings : RefCell<Settings<N>>,
    action   : RefCell<Action<N>>, 
}
impl<'a> BranchB<0> where () : RegisterSelection<0>{
    pub fn new(s: Settings<0>, a:Action<0>)->BranchB<0>{
        Self { settings : RefCell::new(s), action : RefCell::new(a) }
    }
}
//---
pub trait Branchable{
    type Type;
    fn set_duty(&mut self, value:Self::Type);
    fn set_mode(&mut self, mode:PwmMode);
}
impl<'a> Branchable for BranchA<0>{
    type Type = u8;
    fn set_duty(&mut self, value:Self::Type){
        self.action.borrow_mut().set_counter_a(value);
    }
    fn set_mode(&mut self, mode:PwmMode){
        let bits= match mode {
            PwmMode::Normal =>!((1<<Tccr0aBits::COM0A0 as u8)|(1<<Tccr0aBits::COM0A1 as u8)),
            PwmMode::NonInverted =>!(1<<Tccr0aBits::COM0A0 as u8)&(1<<Tccr0aBits::COM0A1 as u8),
            PwmMode::Inverted =>(1<<Tccr0aBits::COM0A0 as u8)|(1<<Tccr0aBits::COM0A1 as u8),
        };
        if let Some(PwmMode::Normal) = Some(&mode){
            self.settings.borrow_mut().tccra().modify(|reg| reg & bits);
        }else{
            self.settings.borrow_mut().tccra().modify(|reg| reg | bits);
        }
    }
}
impl<'a> Branchable for BranchB<0>{
    type Type = u8;
    fn set_duty(&mut self, value:Self::Type){
        self.action.borrow_mut().set_counter_b(value);
    }
    fn set_mode(&mut self, mode:PwmMode){
        let bits= match mode {
            PwmMode::Normal =>!((1<<Tccr0aBits::COM0B0 as u8)|(1<<Tccr0aBits::COM0B1 as u8)),
            PwmMode::NonInverted =>!(1<<Tccr0aBits::COM0B0 as u8)&(1<<Tccr0aBits::COM0B1 as u8),
            PwmMode::Inverted =>(1<<Tccr0aBits::COM0B0 as u8)|(1<<Tccr0aBits::COM0B1 as u8),
        };
        if let Some(PwmMode::Normal) = Some(&mode){
            self.settings.borrow_mut().tccra().modify(|reg| reg & bits);
            
        }else{
            self.settings.borrow_mut().tccra().modify(|reg| reg | bits);
        }
    }
}