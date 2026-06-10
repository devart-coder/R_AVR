use super::*;
use crate::utils::register::*;
pub struct Output {
    ucsr0a: Register,
    udr0: Register,
}
impl Output {
    pub const fn new() -> Self {
        Output {
            ucsr0a: Register::new(UartRegisters::UCSR0A as u8),
            udr0: Register::new(UartRegisters::UDR0 as u8),
        }
    }
    pub fn send_slice(&self, string: &str) -> &Self {
        self.send_bytes(string.as_bytes())
    }
    pub fn send_bytes(&self, bytes: &[u8]) -> &Self {
        for byte in bytes {
            while (self.ucsr0a.read() & (1 << Ucsr0aBits::UDRE0 as u8)) == 0 {}
            self.udr0.write(*byte);
        }
        self
    }
    pub fn send_slice_ln(&self, string: &str) -> &Self {
        self.send_slice(string).send_slice("\n")
    }
    pub fn send_number<T>(&self, v: T) -> &Self
    where
        T: itoa::Integer,
    {
        let mut num_buff = itoa::Buffer::new();
        self.send_slice(num_buff.format(v))
    }
    pub fn send_bool(&self, value: bool) -> &Self {
        let result = match value {
            true => "true",
            false => "false",
        };
        self.send_slice(result)
    }
}
