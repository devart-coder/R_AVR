pub enum Timsk2Bits{
    Toie2  = 0,
    Ocie2a = 1,
    Ocie2b = 2,
}
impl Timsk2Bits{
    const TOIR2:u8  = Timsk2Bits::Toie2  as u8;
    const OCIE2A:u8 = Timsk2Bits::Ocie2a as u8;
    const OCIE2B:u8 = Timsk2Bits::Ocie2b as u8;
}

