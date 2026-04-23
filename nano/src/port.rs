use crate::utils::register::Register;
use crate::mcu::registers::Ports;
macro_rules! define_port {
    ($struct_name:ident, $pin:expr) => {
        #[repr(C)]
        pub struct $struct_name {
            pub pin : Register<u8>,
            pub ddr : Register<u8>,
            pub port: Register<u8>,
        }
        impl $struct_name {
            pub const fn new()->Self{
                Self{
                    pin: Register::new($pin),
                    ddr: Register::new($pin+1),
                    port:Register::new($pin+2),
                }
            }
        }
    };
}

define_port!(PortB, Ports::PortB as u8);
define_port!(PortC, Ports::PortC as u8);
define_port!(PortD, Ports::PortD as u8);

pub const PORTB:PortB = PortB::new();
pub const PORTC:PortC = PortC::new();
pub const PORTD:PortD = PortD::new();
