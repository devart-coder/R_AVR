// use super::action_trait::ActionTrait;
use super::action::Action;
use super::callback::Callback;
use super::settings::*;
use crate::utils::register::*;
use super::interrupt::*;
static mut CALL_BACKS:Callback = Callback::new();
pub struct Timer <const N:u8> where(): RegisterSelection<N>{
    pub settings  : Settings<N>,
	pub callback  : *mut Callback,
	pub action    : Action<N>,
	pub interrupt : Interrupt<N>,
}
impl Timer<0>{
    pub const fn new()->Self{
	    Self{
		    settings: Settings::<0>::new().unwrap(),
			callback: core::ptr::addr_of_mut!(CALL_BACKS) ,
			action: Action::<0>::new(),
			interrupt : Interrupt::<0>::new(),
		}
	}
}
impl Timer<2>{
    pub const fn new()->Self{
	    Self{
		    settings: Settings::<2>::new().unwrap(),
			callback: core::ptr::addr_of_mut!(CALL_BACKS) ,
			action: Action::<2>::new(),
			interrupt : Interrupt::<2>::new(),
		}
	}
}
impl Timer<1>{
    pub const fn new()->Self{
	    Self{
		    settings: Settings::<1>::new().unwrap(),
			callback: core::ptr::addr_of_mut!(CALL_BACKS),
			action: Action::<1>::new(),
			interrupt : Interrupt::<1>::new(),
		}
	}
}
pub const TIMER_0:Timer<0> = Timer::<0>::new();
pub const TIMER_1:Timer<1> = Timer::<1>::new();
pub const TIMER_2:Timer<2> = Timer::<2>::new();
