use crate::utils::callable::CallBacks;
pub struct Callback{
    _callback:CallBacks<3,fn()>,
}
impl Callback{
    pub const fn new()->Self{
	    Self{ _callback:CallBacks::<3,fn()>::new(), }
	}
	pub unsafe fn channal_a(&mut self,f:fn()){
	    self._callback.set_callback(0, f);
	}
	pub fn channal_b(&mut self,f:fn()){
	    self._callback.set_callback(1, f);
	}
	pub fn overflow(&mut self,f:fn()){
	    self._callback.set_callback(2, f);
	}
	pub fn channal_a_callback(&self)->Option<fn()>{
	    self._callback.call(0)
	}
	pub fn channal_b_callback(&self)->Option<fn()>{
	    self._callback.call(1)
	}
	pub fn ovf_callback(&self)->Option<fn()>{
	    self._callback.call(2)
	}
}
