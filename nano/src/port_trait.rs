use super::utils::register::Register;
use crate::mcu::registers::Ports;
use super::port::*;

pub trait PortTrait{
    fn pin()  -> Register<u8>;
    fn ddr()  -> Register<u8>;
    fn port() -> Register<u8>;
}

impl PortTrait for PortB{
    fn pin() -> Register<u8>{
        Register::new(Ports::PortB as u8)
    }
    fn ddr() -> Register<u8>{
        Register::new((Ports::PortB as u8)+1)
    }
    fn port() -> Register<u8>{
        Register::new((Ports::PortB as u8)+2)
    }
}

impl PortTrait for PortC{
    fn pin() -> Register<u8>{
        Register::new(Ports::PortC as u8)
    }
    fn ddr() -> Register<u8>{
        Register::new((Ports::PortC as u8)+1)
    }
    fn port() -> Register<u8>{
        Register::new((Ports::PortC as u8)+2)
    }
}

impl PortTrait for PortD{
    fn pin() -> Register<u8>{
        Register::new(Ports::PortD as u8)
    }
    fn ddr() -> Register<u8>{
        Register::new((Ports::PortD as u8)+1)
    }
    fn port() -> Register<u8>{
        Register::new((Ports::PortD as u8)+2)
    }
}
