use crate::utils::{callable::CallBacks,register::Register};
use crate::mcu::registers::*;
use super::{interrupt::Interrupt, settings::Settings};
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
        let string = match value{
            true => "true",
            false => "false",
        };
        self.send_slice(string);
    }
}

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

pub struct Uart{
    pub settings:Settings,
    pub interrupt:Interrupt,
    pub output:Output,
    pub input:Input,
}
impl Uart{
    pub const fn new()->Self{
        Uart {
            settings: Settings::new(),
            interrupt: Interrupt::new(),
            output: Output::new(),
            input:Input::new(),
        }
    }
}

pub const uart:Uart = Uart::new();


