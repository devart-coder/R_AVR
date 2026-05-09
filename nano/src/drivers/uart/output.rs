use crate::utils::register::*;
use super::*;
pub struct Output{
    ucsr0a:Register,
    udr0:Register,
}
impl Output{
    pub const fn new()->Self{
        Output {
            ucsr0a: Register::new(UartRegisters::UCSR0A as u8),
            udr0: Register::new(UartRegisters::UDR0 as u8),
        }
    }
    pub fn send_slice(&self, string:&str){
        for byte in string.as_bytes(){
            while (self.ucsr0a.read() & (1 << Ucsr0aBits::UDRE0 as u8)) == 0 {}
            self.udr0.write(*byte);
        }
    }
    pub fn send_bool(&self, value:bool){
        let result = match value{
            true => "true",
            false => "false",
        };
        self.send_slice(result);
    }
}
