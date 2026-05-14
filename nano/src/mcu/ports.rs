use crate::utils::register::Register;
use crate::mcu::registers::Ports;

pub struct PortB;
pub struct PortC;
pub struct PortD;

pub trait Portable{
    fn pin()  -> Register;
    fn ddr()  -> Register;
    fn port() -> Register;
}

impl Portable for PortB{
    fn pin() -> Register{ Register::new(Ports::PortB as u8) }
    fn ddr() -> Register{ Register::new((Ports::PortB as u8)+1) }
    fn port() -> Register{ Register::new((Ports::PortB as u8)+2) }
}

impl Portable for PortC{
    fn pin()  -> Register{ Register::new(Ports::PortC as u8) }
    fn ddr()  -> Register{ Register::new((Ports::PortC as u8)+1) }
    fn port() -> Register{ Register::new((Ports::PortC as u8)+2) }
}

impl Portable for PortD{
    fn pin() -> Register{ Register::new(Ports::PortD as u8) }
    fn ddr() -> Register{ Register::new((Ports::PortD as u8)+1) }
    fn port() -> Register{ Register::new((Ports::PortD as u8)+2) }
}
