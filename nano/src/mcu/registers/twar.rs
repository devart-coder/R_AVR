pub struct Twar;
impl Twar{
    pub fn address() -> u8 { 0xBA }
}

pub enum TwarBits{} 
impl TwarBits{
    const TWGE:u8 = 0;//TWI General Call Recognition Enable Bit
    const TWA0:u8 = 1;//TWI (Slave) Address Register
    const TWA1:u8 = 2;//TWI (Slave) Address Register
    const TWA2:u8 = 3;//TWI (Slave) Address Register
    const TWA3:u8 = 4;//TWI (Slave) Address Register
    const TWA4:u8 = 5;//TWI (Slave) Address Register
    const TWA5:u8 = 6;//TWI (Slave) Address Register
    const TWA6:u8 = 7;//TWI (Slave) Address Register
}