#[repr(u8)]
pub enum ExIntRegisters{
    Eicra = 0x69,
    Eifr  = 0x1C,
    Eimsk = 0x1D,
}
impl ExIntRegisters{
    const EICRA:u8 = Self::Eicra as u8;
    const EIFR:u8 = Self::Eifr as u8;
    const EIMSK:u8 = Self::Eimsk as u8;
}
//---
#[repr(u8)]
pub enum Ports{
    PortB = 0x23,
    PortC = 0x26,
    PortD = 0x29
}
impl Ports {
    const PORTB:u8 = Self::PortB as u8;
    const PORTC:u8 = Self::PortC as u8;
    const PORTD:u8 = Self::PortD as u8;
}
//---
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
