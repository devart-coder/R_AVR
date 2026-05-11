use super::action::Action;
use super::callback::Callback;
use super::settings::*;
use super::interrupt::*;
use crate::drivers::timers::RegisterSelection;
use crate::drivers::pwm::pwm::*;
static mut CALL_BACKS:Callback = Callback::new();
pub struct Timer <const N:u8> where (): RegisterSelection<N>{
    settings  : Settings<N>,
	callback  : *mut Callback,
	action    : Action<N>,
	interrupt : Interrupt<N>,
}
unsafe impl<const N: u8> Sync for Timer<N> where(): RegisterSelection<N>{}
macro_rules! impl_timer {
    ($index:expr) => {
	    impl Timer<$index>{
		    const fn new()->Self{
			    Self{
				    settings: Settings::<$index>::new().unwrap(),
					callback: core::ptr::addr_of_mut!(CALL_BACKS) ,
					action: Action::<$index>::new(),
					interrupt : Interrupt::<$index>::new(),
				}
			}
			pub fn get() -> &'static mut Self{
			    static mut T0:Timer<$index> = Timer::<$index>::new();
				unsafe { &mut *core::ptr::addr_of_mut!(T0) }
			}
			pub fn callback(&mut self)->&mut Callback{
			    unsafe {&mut *self.callback}
			}
			pub fn interrupt(&mut self)->&mut Interrupt<$index>{
			    &mut self.interrupt
			}
			pub fn action(&mut self)->&mut Action<$index>{
			    &mut self.action
			}
			pub fn settings(&mut self)->&mut Settings<$index>{
			    &mut self.settings
			}
			pub fn into_pwm(&mut self)->Pwm<$index>{
        			self.settings().set_mode(Mode::PwdFast);
				Pwm::new(Timer::<$index>::get())		
			}
		}
	};
}
impl_timer!(0);
impl_timer!(1);
impl_timer!(2);