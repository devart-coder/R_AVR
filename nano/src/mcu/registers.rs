#[repr(u8)]
pub enum Ports{
    PortB = 0x23,
    PortC = 0x26,
    PortD = 0x29
}
#[repr(u8)]
pub enum TimerRegisters{
    TCNT0 =0x46,
    TCCR0B=0x45,
    TCCR0A=0x44,


    TCCR1C=0x82,
    TCCR1B=0x81,
    TCCR1A=0x80,

    TCNT2 =0xB2,
    TCCR2B=0xB1,
    TCCR2A=0xB0,

    OCR1BH=0x8B,
    OCR1BL=0x8A,
    OCR1AH=0x89,
    OCR1AL=0x88,
}
#[repr(u8)]
pub enum Tccr0bBits{
    CS00  = 0,
    CS01  = 1,
    CS02  = 2,
    WGM02 = 3,
    FOC0B = 6,
    FOC0A = 7
}
#[repr(u8)]
pub enum Tccr0aBits{
    WGM00  = 0,
    WGM01  = 1,
    COM0B0 = 4,
    COM0B1 = 5,
    COM0A0 = 6,
    COM0A1 = 7,
}
#[repr(u8)]
pub enum Tccr1aBits{
    WGM10  = 0,
    WGM11  = 1,
    COM1B0 = 4,
    COM1B1 = 5,
    COM1A0 = 6,
    COM1A1 = 7,
}
#[repr(u8)]
pub enum Tccr1bBits{
    CS10=0,
    CS11=1,
    CS12=2,
    WGM12=3,
    WGM13=4,
    ICES1=6,
    ICNC1=7,
}
#[repr(u8)]
pub enum Tccr2aBits{
    WGM20  = 0,
    WGM21  = 1,
    COM2B0 = 4,
    COM2B1 = 5,
    COM2A0 = 6,
    COM2A1 = 7,
}
#[repr(u8)]
pub enum Tccr2bBits{
    CS20=0,
    CS21=1,
    CS22=2,
    WGM22=3,
    FOC2B=6,
    FOC2A=7,
}
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
