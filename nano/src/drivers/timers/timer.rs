use super::action::Action;
use super::callback::Callback;
use super::settings::*;
use crate::utils::register::*;
use super::interrupt::*;
static mut CALL_BACKS:Callback = Callback::new();
pub struct Timer <const N:u8> where(): RegisterSelection<N>{
    pub settings  : Settings<N>,
	callback  : *mut Callback,
	pub action    : Action<N>,
	pub interrupt : Interrupt<N>,
}
unsafe impl<const N: u8> Sync for Timer<N> where(): RegisterSelection<N>{}
macro_rules! impl_timer {
    ($index:expr) => {
	    impl Timer<$index>{
		    pub const fn new()->Self{
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
		}
	};
}
impl_timer!(0);
impl_timer!(1);
impl_timer!(2);
