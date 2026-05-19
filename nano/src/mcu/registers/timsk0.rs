pub enum Timsk0Bits{
    Toie0 = 0,
    Ocie0a = 1,
    Ocie0b = 2,
}
impl Timsk0Bits{
    const TOIE0:u8 = Timsk0Bits::Toie0 as u8;
    const OCIE0A:u8 = Timsk0Bits::Ocie0a as u8;
    const OCIE0B:u8 = Timsk0Bits::Ocie0b as u8;
}