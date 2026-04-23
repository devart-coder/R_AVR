use crate::mcu::registers::*;
use crate::utils::{callable::CallBacks,register::Register};
pub enum Prescaling {
    NoSource = 0,
	_1 = 1,
	_8 = 2,
	_64 = 3,
	_256 = 4,
	_1024 = 5,
	ExternalClockFalling = 6,
	ExternalClockRising = 7,
}
pub enum Mode {
    Normal,
	CTC,
	PwdFast,
	PwdPhaseCorrect,
}
pub struct Settings<const T:u8>{
    _prescaling:Prescaling,
	_tccrb:Register<u8>,
	_tccra:Register<u8>,
}
impl<const T:u8> Settings<T>{
    pub const fn new() -> Option<Self>{
	    match T{
		    0 =>Some(
			    Settings{
				    _tccra:Register::new(TimerRegisters::TCCR0A as u8),
					_tccrb:Register::new(TimerRegisters::TCCR0B as u8),
					_prescaling: Prescaling::NoSource,
				}
			),
			1 =>Some(
			    Settings{
				    _tccra:Register::new(TimerRegisters::TCCR1A as u8),
					_tccrb:Register::new(TimerRegisters::TCCR1B as u8),
					_prescaling: Prescaling::NoSource,
				}
			),
			2 =>Some(
			    Settings{
				    _tccra:Register::new(TimerRegisters::TCCR2A as u8),
					_tccrb:Register::new(TimerRegisters::TCCR2B as u8),
					_prescaling: Prescaling::NoSource,
				}
			),
			_ =>None,
		}
	}
	pub fn prescaling(&mut self, p:Prescaling){
	    let value:u8 = match p {
		    Prescaling::NoSource =>{
			    match T {
				    0=> !((1<<Tccr0bBits::CS02 as u8)|(1<<Tccr0bBits::CS01 as u8)|(1<<Tccr0bBits::CS00 as u8)),
					1=> !((1<<Tccr1bBits::CS12 as u8)|(1<<Tccr1bBits::CS11 as u8)|(1<<Tccr1bBits::CS10 as u8)),
					2=> !((1<<Tccr2bBits::CS22 as u8)|(1<<Tccr2bBits::CS21 as u8)|(1<<Tccr2bBits::CS20 as u8)),
					_=> 0,
				}
			},
			Prescaling::_1 =>{
			    match T {
				    0=> 1<<Tccr0bBits::CS00 as u8,
					1=> 1<<Tccr1bBits::CS10 as u8,
					2=> 1<<Tccr2bBits::CS20 as u8,
					_=> 0,
				}
			},
			Prescaling::_8 =>{
			    match T {
				    0=> 1<<Tccr0bBits::CS01 as u8,
					1=> 1<<Tccr1bBits::CS11 as u8,
					2=> 1<<Tccr2bBits::CS21 as u8,
					_=> 0,
				}
			},
			Prescaling::_64 =>{
			    match T {
				    0=> (1<<Tccr0bBits::CS01 as u8)|(1<<Tccr0bBits::CS00 as u8),
					1=> (1<<Tccr1bBits::CS11 as u8)|(1<<Tccr1bBits::CS10 as u8),
					2=> (1<<Tccr2bBits::CS21 as u8)|(1<<Tccr2bBits::CS20 as u8),
					_=> 0,
				}
			},
			Prescaling::_256=>{
			    match T {
				    0=> 1<<Tccr0bBits::CS02 as u8,
					1=> 1<<Tccr1bBits::CS12 as u8,
					2=> 1<<Tccr2bBits::CS22 as u8,
					_=> 0,
				}
			},
			Prescaling::_1024=>{
			    match T {
				    0=> (1<<Tccr0bBits::CS02 as u8)|(1<<Tccr0bBits::CS00 as u8),
					1=> (1<<Tccr1bBits::CS12 as u8)|(1<<Tccr1bBits::CS10 as u8),
					2=> (1<<Tccr2bBits::CS22 as u8)|(1<<Tccr2bBits::CS20 as u8),
					_=> 0,
				}
			},
			//TODO::applyImpl
			Prescaling::ExternalClockFalling=>{
			    match T{
				    _=> 0u8,
				}
			},
			//TODO::applyImpl
			Prescaling::ExternalClockRising=>{
			    match T{
				    _=> 0u8,
				}
			},
		};
		if let Some(Prescaling::NoSource) = Some(&p) {
		    self._tccrb.modify(|w| w&value);
		}else{
		    self._tccrb.modify(|w| w|value);
		}
		self._prescaling = p;
	}
	pub fn set_mode(&self,mode:Mode){
	    let n:u8 = match mode{
		    Mode::Normal=>{
			    match T{
				    0=>!((1<<Tccr0aBits::WGM00 as u8)|(1<<Tccr0aBits::WGM01 as u8)),
					1=>!((1<<Tccr1aBits::WGM10 as u8)|(1<<Tccr1aBits::WGM11 as u8)),
					2=>!((1<<Tccr2aBits::WGM20 as u8)|(1<<Tccr2aBits::WGM21 as u8)),
					_=>0,
				}
            },
			Mode::CTC=>{
			    match T{
				    0=>1<<Tccr0aBits::WGM01 as u8,
					1=>1<<Tccr1aBits::WGM11 as u8,
					2=>1<<Tccr2aBits::WGM21 as u8,
					_=>0,
				}
            },
			Mode::PwdFast=>{
			    match T{
				    0=>(1<<Tccr0aBits::WGM00 as u8)|(1<<Tccr0aBits::WGM01 as u8),
					1=>(1<<Tccr1aBits::WGM10 as u8)|(1<<Tccr1aBits::WGM11 as u8),
					2=>(1<<Tccr2aBits::WGM20 as u8)|(1<<Tccr2aBits::WGM21 as u8),
					_=>0,
				}
            },
			Mode::PwdPhaseCorrect=>{
			    match T{
				    0=>1<<Tccr0aBits::WGM00 as u8,
					1=>1<<Tccr1aBits::WGM10 as u8,
					2=>1<<Tccr2aBits::WGM20 as u8,
					_=>0,
				}
            },
		};
		self._tccra.modify(|w| w|n);
	}
	pub fn default(&mut self){
	    self.prescaling(Prescaling::_64);
		self.set_mode(Mode::Normal);
	}
}
pub struct Callback{
    _callback:CallBacks<2,fn()>,
}
impl Callback{
    pub const fn new()->Self{
	    Self{ _callback:CallBacks::new(), }
	}
	pub fn channal_a(&mut self,f:fn(fn())){
	    self._callback.set_callback(0, f);
	}
	pub fn channal_b(&mut self,f:fn(fn())){
	    self._callback.set_callback(1, f);
	}
	pub fn channal_a_callback(&self)->Option<fn(fn())>{
	    self._callback.call(0)
	}
	pub fn channal_b_callback(&self)->Option<fn(fn())>{
	    self._callback.call(1)
	}
}

pub struct Timer <const T:u8> {
    pub settings:Settings<T>,
	pub interrupt : Callback,
}
impl<const T:u8> Timer<T>{
    pub const fn new()->Self{
	    Self{
		    settings: Settings::<T>::new().unwrap(),
			interrupt: Callback::new(),
		}
	}
}
pub const TIMER_0:Timer<0> = Timer::<0>::new();
pub const TIMER_1:Timer<1> = Timer::<1>::new();
pub const TIMER_2:Timer<2> = Timer::<2>::new();
