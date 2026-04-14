use crate::register::Register;

pub struct Port {
    pub ddr:Register,
    pub port:Register,
}
impl Port{
    pub fn new()->Self{
        Port{
            ddr:  Register{ address:0x24 as *mut u8 },
            port: Register{ address:0x25 as *mut u8 }
        }
    }
}
