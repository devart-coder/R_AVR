use crate::register::Register;
enum PortName{
    B=0x23,
    C=0x26,
    D=0x29
}

macro_rules! define_port {
    ($struct_name:ident, $pin:expr) => {
        #[repr(C)]
        pub struct $struct_name {
            pub pin:Register<u8>,
            pub ddr:Register<u8>,
            pub port:Register<u8>,
        }
        impl $struct_name {
            pub const fn new() -> Self{
                Self{
                    pin: Register::new($pin),
                    ddr: Register::new($pin+1),
                    port:Register::new($pin+2),
                }
            }
        }
    };
}
define_port!(PortB, PortName::B as usize);
define_port!(PortC, PortName::C as usize);
define_port!(PortD, PortName::D as usize);
pub const PORTB:PortB = PortB::new();
pub const PORTC:PortC = PortC::new();
pub const PORTD:PortD = PortD::new();
