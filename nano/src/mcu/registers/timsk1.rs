pub enum Timsk1Bits{
    Toie1  = 0,
    Ocie1a = 1,
    Ocie1b = 2,
    Icie1  = 5,
}
impl Timsk1Bits{
    const TOIE1:u8 = Timsk1Bits::Toie1 as u8;
    const OCIE1A:u8 = Timsk1Bits::Ocie1a as u8;
    const OCIE1B:u8 = Timsk1Bits::Ocie1b as u8;
    const ICIE1:u8 = Timsk1Bits::Icie1 as u8;
}