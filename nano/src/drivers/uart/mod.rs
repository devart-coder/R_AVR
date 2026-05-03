pub mod action;
pub mod interrupt;
// pub mod callback;
pub mod settings;
pub mod uart;
#[repr(u8)]
pub enum UartRegisters{
    UCSR0A = 0xC0,
    UCSR0C = 0xC2,
    UCSR0B = 0xC1,
    UBRR0H = 0xC5,
    UBRR0L = 0xC4,
    UDR0   = 0xC6,
}
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
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
