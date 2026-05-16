#[repr(u8)]
pub enum Ports{
    PortB = 0x23,
    PortC = 0x26,
    PortD = 0x29
}
#[repr(u8)]
pub enum TimerRegisters{
    TCNT0 =0x46,
    TCNT1H=0x85,
    TCNT1L=0x84,
    TCNT2 =0xB2,

    TCCR0A=0x44,
    TCCR0B=0x45,
    TCCR1A=0x80,
    TCCR1B=0x81,
    TCCR1C=0x82,
    TCCR2A=0xB0,
    TCCR2B=0xB1,

    OCR0A=0x47,
    OCR0B=0x48,
    OCR1AH=0x89,
    OCR1AL=0x88,
    OCR1BH=0x8B,
    OCR1BL=0x8A,
    OCR2B=0xB4,
    OCR2A=0xB3,

    TIMSK0=0x6E,
    TIMSK1=0x6F,
    TIMSK2=0x70,

}
pub enum Timsk0Bits{
    TOIE0 = 0,
    OCIE0A = 1,
    OCIE0B = 2,
}
pub enum Timsk1Bits{
    TOIE1 = 0,
    OCIE1A = 1,
    OCIE1B = 2,
    ICIE1 = 5,
}
pub enum Timsk2Bits{
    TOIE2 = 0,
    OCIE2A = 1,
    OCIE2B = 2,
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
