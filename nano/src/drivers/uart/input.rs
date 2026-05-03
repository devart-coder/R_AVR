use crate::utils::register::*;
use super::*;
pub struct Input{
    ucsr0a:Register,
    udr0:Register,
}
impl Input {
    pub const fn new()->Self{
        Input {
            ucsr0a: Register::new(UartRegisters::UCSR0A as u8),
            udr0:	Register::new(UartRegisters::UDR0 	as u8),
        }
    }
    pub fn receive_byte(&self)->u8{
        while (self.ucsr0a.read() & (1 << Ucsr0aBits::RXC0 as u8)) == 0{}
        self.udr0.read()
    }
    pub fn receive_byte_slice(&self)->[u8;1]{
        [self.receive_byte()]//make a byte slice
    }
}
