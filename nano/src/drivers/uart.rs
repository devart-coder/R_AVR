use crate::utils::{callable::CallBacks,register::Register};
use crate::mcu::registers::*;
#[repr(u32)]
pub enum BaudRate{
    _300=300,
    _1200=1200,
    _2400=2400,
    _4800=4800,
    _9600=9600,
    _19200=19200,
    _38400=38400,
    _57600=57600,
    _115200=115200,
}
pub enum PackageBits{
    _5,
    _6,
    _7,
    _8,
    _9,
}
pub enum ParityMode{
    Disabled,
    Even,
    Odd
}
pub enum StopBits{
    _1,
    _2
}

pub struct Interrupt{
    ucsr0b: Register,
    call_array : CallBacks<2,fn()>,
}
impl Interrupt{
    pub const fn new()->Self{
        Interrupt {
            ucsr0b: Register::new(UartRegisters::UCSR0B as u8),
            call_array : CallBacks::<2,fn()>::new(),
        }
    }
    pub fn set_rx_handle(&mut self, f:fn()){
        self.call_array.set_callback(0,f);
    }
    pub fn set_tx_handle(&mut self, f:fn()){
        self.call_array.set_callback(1,f);
    }
    pub fn handle_rx(&self) -> Option<fn()>{
        self.call_array.call(0)
    }
    pub fn handle_tx(&self) -> Option<fn()>{
        self.call_array.call(1)
    }
    pub fn enable_rxi(&self, value:bool ) {
        if value == true{
            self.ucsr0b.modify(|v| v|(1<<Ucsr0bBits::RXCIE0 as u8)|(1<<Ucsr0bBits::RXEN0 as u8));
        }else{
            self.ucsr0b.modify(|v| v&!((1<<Ucsr0bBits::RXCIE0 as u8)|(1<<Ucsr0bBits::RXEN0 as u8)));
        }
    }
    pub fn enable_txi(&self, value:bool ) {
        if value == true{
            self.ucsr0b.modify(|v| v | (1<<Ucsr0bBits::TXCIE0 as u8)|(1<<Ucsr0bBits::TXEN0 as u8));
        }else{
            self.ucsr0b.modify(|v| v & !((1<<Ucsr0bBits::TXCIE0 as u8)|(1<<Ucsr0bBits::TXEN0 as u8)));
        }
    }
}

pub struct Settings{
    ucsr0c:Register,
    ubrr0h:Register,
    ubrr0l:Register,
    pub interrupt : Interrupt,
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
            PackageBits::_5 =>
                self.ucsr0c.modify(|value| value & !((1<<Ucsr0cBits::UCSZ00 as u8)|(1<<Ucsr0cBits::UCSZ01 as u8))),
            PackageBits::_6 =>
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ00 as u8)),
            PackageBits::_7 =>
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ01 as u8)),
            PackageBits::_8 =>
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ00 as u8)|(1<<Ucsr0cBits::UCSZ01 as u8)),
            PackageBits::_9 => {
                self.interrupt.ucsr0b.modify(|value| value | (1<<Ucsr0bBits::UCSZ02 as u8));
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ00 as u8)|(1<<Ucsr0cBits::UCSZ01 as u8));
            }
        }
    }
    pub fn parity_mode(&self, value:ParityMode){
        match value {
            ParityMode::Disabled =>{
                self.ucsr0c.modify(|value|value&!((1<<Ucsr0cBits::UPM00 as u8)|(1<<Ucsr0cBits::UPM00 as u8)));
            }
            ParityMode::Even =>{
                self.ucsr0c.modify(|value|value | (1<<Ucsr0cBits::UPM00 as u8));
            }
            ParityMode::Odd =>{
                self.ucsr0c.modify(|value|value | (1<<Ucsr0cBits::UPM00 as u8)|(1<<Ucsr0cBits::UPM00 as u8));
            }
        }
    }
    pub fn stop_bits(&self, value:StopBits){
        match value {
            StopBits::_1=>{
                self.ucsr0c.modify(|reg| reg & !(1 << Ucsr0cBits::USBS0 as u8));
            }
            StopBits::_2=>{
                self.ucsr0c.modify(|reg| reg | (1 << Ucsr0cBits::USBS0 as u8));
            }
        }
    }
    pub fn baud_rate(&self, value:BaudRate){
        //TODO::NeedRemake
        const F_CPU:u32 =16_000_000;
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
    pub fn default(&self){
        self.baud_rate(BaudRate::_9600);
        self.package_bits(PackageBits::_8);
        self.parity_mode(ParityMode::Disabled);
        self.stop_bits(StopBits::_1);
        self.interrupt.enable_rxi(true);
        self.interrupt.enable_txi(true);
    }
}

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
