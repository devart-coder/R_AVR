use super::register::Register;
use super::port::*;
pub trait PortTrait{
    fn pin() -> Register<u8>;
    fn ddr() -> Register<u8>;
    fn port() -> Register<u8>;
}
impl PortTrait for PortB{
    fn pin() -> Register<u8>{
        Register::new(0x23)
    }
    fn ddr() -> Register<u8>{
        Register::new(0x24)
    }
    fn port() -> Register<u8>{
        Register::new(0x25)
    }
}
impl PortTrait for PortC{
    fn pin() -> Register<u8>{
        Register::new(0x26)
    }
    fn ddr() -> Register<u8>{
        Register::new(0x26+1)
    }
    fn port() -> Register<u8>{
        Register::new(0x26+2)
    }
}
impl PortTrait for PortD{
    fn pin() -> Register<u8>{
        Register::new(0x29)
    }
    fn ddr() -> Register<u8>{
        Register::new(0x29+1)
    }
    fn port() -> Register<u8>{
        Register::new(0x29+2)
    }
}
