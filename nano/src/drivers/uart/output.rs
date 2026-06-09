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
    pub fn send_slice(&self, string: &str) {
        for byte in string.as_bytes() {
            while (self.ucsr0a.read() & (1 << Ucsr0aBits::UDRE0 as u8)) == 0 {}
            self.udr0.write(*byte);
        }
    }
    pub fn send_u16_be(&self, v: u16) {
        let [high, low] = v.to_be_bytes();
        self.send_u8(high);
        self.send_u8(low);
    }
    fn reverse(&self, v: u8) -> u16 {
        let mut value = v;
        let mut result: u16 = 0;
        while value != 0 {
            let div: u16 = value as u16 % 10;
            result = result * 10 + div;
            value /= 10;
        }
        result
    }
    pub fn send_u8(&self, v: u8) {
        let mut div = 0u16;
        let mut value = self.reverse(v);
        while value != 0 {
            div = value % 10 + b'0' as u16;
            self.send_slice(str::from_utf8(&[div as u8]).unwrap());
            value /= 10;
        }
    }
    pub fn send_number(&self, value: u8) {
        self.send_slice(str::from_utf8(&[value as u8]).unwrap());
    }
    pub fn send_bool(&self, value: bool) {
        let result = match value {
            true => "true",
            false => "false",
        };
        self.send_slice(result);
    }
}
