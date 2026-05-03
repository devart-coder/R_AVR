use crate::utils::register::Register;
use super::interrupt::*;
use super::*;
use crate::F_CPU;
use super::builders::*;
pub struct Settings{
    ucsr0c:Register,
    ubrr0h:Register,
    ubrr0l:Register,
    interrupt : Interrupt,
}
impl Settings{
    pub const fn new()-> Self{
        Settings {
            ucsr0c: Register::new(UartRegisters::UCSR0C as u8),
            ubrr0h: Register::new(UartRegisters::UBRR0H as u8),
            ubrr0l: Register::new(UartRegisters::UBRR0L as u8),
            interrupt: Interrupt::new(),
        }
    }
    pub fn double_speed(&self, value:bool){
        let ucsr0a = Register::new(UartRegisters::UCSR0A as u8);
        if value == true {
            ucsr0a.modify(|reg| reg|(1<<Ucsr0aBits::U2X0 as u8));
        }else{
            ucsr0a.modify(|reg| reg&(1<<Ucsr0aBits::U2X0 as u8));
        }
    }
    pub fn package_bits(&self, value: PackageBits){
        match value {
            PackageBits::_5 => self.ucsr0c.modify(|v| v|Ucsr0cBuilder(0).dis_ucsz00().dis_ucsz01().build()),
            PackageBits::_6 => self.ucsr0c.modify(|v| v|Ucsr0cBuilder(0).en_ucsz00().build()),
            PackageBits::_7 => self.ucsr0c.modify(|v| v|Ucsr0cBuilder(0).en_ucsz01().build()),
            PackageBits::_8 => self.ucsr0c.modify(|v| v|Ucsr0cBuilder(0).en_ucsz00().en_ucsz01().build()),
            PackageBits::_9 => {
                // self.interrupt.ucsr0b.modify(|value| value | (1<<Ucsr0bBits::UCSZ02 as u8));
            }
        }
    }
    pub fn parity_mode(&self, value:ParityMode){
        match value {
            ParityMode::Disabled => self.ucsr0c.modify(|v| v| Ucsr0cBuilder(0).dis_upm00().dis_upm01().build()), 
            ParityMode::Even     => self.ucsr0c.modify(|v| v| Ucsr0cBuilder(0).en_upm00().build()),
            ParityMode::Odd      => self.ucsr0c.modify(|v| v| Ucsr0cBuilder(0).en_upm00().en_upm01().build()),
        }
    }
    pub fn stop_bits(&self, value:StopBits){
        match value {
            StopBits::_1=> self.ucsr0c.modify(|v| v| Ucsr0cBuilder(0).dis_usbs0().build()),
            StopBits::_2=> self.ucsr0c.modify(|v| v| Ucsr0cBuilder(0).en_usbs0().build()),
        }
    }
    pub fn baud_rate(&self, value:BaudRate){
        let speed:u32 = match value{
            BaudRate::_115200=>{
                self.double_speed(true);
                8
            }
            _=>{
                self.double_speed(false);
                16
            }
        };
        let speed=(F_CPU/(speed*(value as u32)))-1;
        self.ubrr0h.write((speed>>8)as u8);
        self.ubrr0l.write(speed as u8);
    }
    pub fn default(&mut self){
        self.baud_rate(BaudRate::_9600);
        self.package_bits(PackageBits::_8);
        self.parity_mode(ParityMode::Disabled);
        self.stop_bits(StopBits::_1);
        self.interrupt.enable_rxi(true);
        self.interrupt.enable_txi(true);
    }
}
