use super::register::Register;
pub enum UartRegisters{
    UCSR0A = 0xC0,
    UCSR0C = 0xC2,
    UCSR0B = 0xC1,
    UBRR0H = 0xC5,
    UBRR0L = 0xC4,
    UDR0   = 0xC6,
}
pub enum Ucsr0aBits{
    MPCM0 = 0,
    U2X0  = 1,
    UPE0  = 2,
    DOR0  = 3,
    FE0   = 4,
    UDRE0 = 5,
    TXC0  = 6,
    RXC0  = 7,
}
pub enum Ucsr0bBits{
    TXB80   = 0,
    RXB80   = 1,
    UCSZ02  = 2,
    TXEN0   = 3,
    RXEN0   = 4,
    UDRIE0  = 5,
    TXCIE0  = 6,
    RXCIE0  = 7,
}
pub enum Ucsr0cBits{
    UCPOL0  = 0,
    UCSZ00  = 1,
    UCSZ01  = 2,
    USBS0   = 3,
    UPM00   = 4,
    UPM01   = 5,
    UMSEL00 = 6,
    UMSEL01 = 7,
}
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
//---
struct Interrupt{
}
impl Interrupt{
}
//---
pub struct Settings{
    ucsr0a:Register<u8>,
    ucsr0b:Register<u8>,
    ucsr0c:Register<u8>,
    ubrr0h:Register<u8>,
    ubrr0l:Register<u8>,
}
impl Settings{
    pub const fn new()-> Self{
        Settings {
            ucsr0a: Register::new(UartRegisters::UCSR0A as usize),
            ucsr0b: Register::new(UartRegisters::UCSR0B as usize),
            ucsr0c: Register::new(UartRegisters::UCSR0C as usize),
            ubrr0h: Register::new(UartRegisters::UBRR0H as usize),
            ubrr0l: Register::new(UartRegisters::UBRR0L as usize),
        }
    }
}
pub trait SettingsTrait{
    fn double_speed(&self, value:bool);
    fn package_bits(&self, value: PackageBits);
    fn parity_mode(&self, value: ParityMode);
    fn stop_bits(&self, value:StopBits);
    fn baud_rate(&self, value:BaudRate);
    fn enable_rxi(&self, value:bool);
    fn enable_txi(&self, value:bool);
    fn default(&self);
}
impl SettingsTrait for Settings{
    fn double_speed(&self, value:bool){
        if value == true {
            self.ucsr0a.modify(|reg| reg|(1<<Ucsr0aBits::U2X0 as u8));
        }else{
            self.ucsr0a.modify(|reg| reg&(1<<Ucsr0aBits::U2X0 as u8));
        }
    }
    fn package_bits(&self, value: PackageBits){
        match value {
            PackageBits::_5 => {
                self.ucsr0c.modify(|value| value & !((1<<Ucsr0cBits::UCSZ00 as u8)|(1<<Ucsr0cBits::UCSZ01 as u8)))
            },
            PackageBits::_6 => {
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ00 as u8))
            }
            PackageBits::_7 => {
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ01 as u8))
            }
            PackageBits::_8 => {
                self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ00 as u8)|(1<<Ucsr0cBits::UCSZ01 as u8))
            }
            PackageBits::_9 => {
                // self.ucsr0c.modify(|value| value | (1<<Ucsr0cBits::UCSZ0 as u8))
            }
        }
    }
    fn parity_mode(&self, value:ParityMode){
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
    fn stop_bits(&self, value:StopBits){
        match value {
            StopBits::_1=>{
                self.ucsr0c.modify(|reg| reg & !(1 << Ucsr0cBits::USBS0 as u8));
            }
            StopBits::_2=>{
                self.ucsr0c.modify(|reg| reg | (1 << Ucsr0cBits::USBS0 as u8));
            }
        }
    }
    fn baud_rate(&self, value:BaudRate){
        //NeedreMake
        let f_cpu:u32 =16_000_000;
        let speed:u32 = match value{
            BaudRate::_115200=>{
                self.double_speed(true);
                (f_cpu/( 8*(value as u32) ))-1
            }
            _=>{
                self.double_speed(false);
                (f_cpu/(16*(value as u32)))-1
            }
        };
            self.ubrr0h.write((speed>>8)as u8);
            self.ubrr0l.write(speed as u8);
    }
    fn default(&self){
        self.baud_rate(BaudRate::_9600);
        self.package_bits(PackageBits::_8);
        self.parity_mode(ParityMode::Disabled);
        self.stop_bits(StopBits::_1);
    }
    fn enable_rxi(&self, value:bool ) {
        if value == true{
            self.ucsr0b.modify(|v| v | (1<<Ucsr0bBits::RXCIE0 as u8) );
        }else{
            self.ucsr0b.modify(|v| v & !(1<<Ucsr0bBits::RXCIE0 as u8) );
        }
    }
    fn enable_txi(&self, value:bool ) {
        if value == true{
            self.ucsr0b.modify(|v| v | (1<<Ucsr0bBits::TXCIE0 as u8) );
        }else{
            self.ucsr0b.modify(|v| v & !(1<<Ucsr0bBits::TXCIE0 as u8) );
        }
    }
}
//---
pub struct Output{

}
//---
pub struct Input{
}
//---
pub struct Uart{
    pub settings:Settings,
    pub out:Output,
    // pub in:Input,
}
impl Uart{
    pub const fn new()->Self{
        Uart {
            settings: Settings::new()
        }
    }
}
pub const uart:Uart = Uart::new();
