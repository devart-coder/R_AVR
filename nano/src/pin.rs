pub struct Input;
pub struct Output;

use super::port_trait::PortTrait;
pub struct Pin<PORT, const BIT:u8, MOD>{
    _port: core::marker::PhantomData<PORT>,
    _mod:  core::marker::PhantomData<MOD>,
}
impl<PORT:PortTrait, const BIT:u8> Pin<PORT, BIT, Input>{
    pub const fn new() -> Self {
        Self {
            _port: core::marker::PhantomData,
            _mod: core::marker::PhantomData,
        }
    }
    pub fn is_high(&self)->bool{
        PORT::pin().read() & (1<<BIT) == (1<<BIT)
    }
    pub fn is_low(&self)->bool{
        !self.is_high()
    }
    pub fn into_output(&self)->Pin<PORT, BIT, Output>{
        PORT::ddr().modify(|reg|reg | (1<<BIT));
        Pin{
            _port:core::marker::PhantomData,
            _mod:core::marker::PhantomData,
        }
    }
}

impl<PORT:PortTrait, const BIT:u8> Pin<PORT, BIT, Output>{
    pub fn set_high(&self){
        PORT::port().modify(|reg| reg | (1<< BIT));
    }
    pub fn set_low(&self){
        PORT::port().modify(|reg| reg & !(1<< BIT));
    }
    pub fn toggle(&self){
        PORT::port().modify(|reg| reg ^ (1<< BIT));
    }
}
