pub struct Register{
    address:*mut u8
}
impl Register{
    pub const fn new(address: u8)->Self{
        Self{address:address as *mut u8}
    }
    //WriteBlock
    pub fn write(&self, value:u8){
        unsafe{
            core::ptr::write_volatile(self.address,value);
        }
    }
    //ReadBlock
    pub fn read(&self)->u8{
        unsafe{
            core::ptr::read_volatile(self.address)
        }
    }
    //ModifyBlock
    pub fn modify<F> (&self, f:F) where F:FnOnce(u8)->u8{
        let value = self.read();
        self.write(f(value));
    }
}
//---DOUBLE---
pub struct RegisterU16{
    pub lreg: Register,
    pub hreg: Register,
}
impl RegisterU16{
    const fn new ()->Self{
        RegisterU16 { lreg: Register::new(0), hreg: Register::new(0) }
    }
    pub fn write(&self, value:u16){
        let bytes = value.to_be_bytes();
        let l = bytes[0] as u8;
        let h = bytes[1] as u8;
        self.lreg.write(l);
        self.hreg.write(h);
    }
    pub fn read(&self)->u16{
        let l = self.lreg.read();
        let h = self.hreg.read();
        u16::from_be_bytes([l,h])
    }
}
pub struct RegisterU16Builder{
    reg:RegisterU16
}
impl RegisterU16Builder{
    pub const fn builder()->Self{
        Self{ reg: RegisterU16::new() }
    }
    pub const fn set_lreg(mut self, value:u8)->Self{
        self.reg.lreg = Register::new(value);
        self
    }
    pub const fn set_hreg(mut self,value:u8)->Self{
        self.reg.hreg = Register::new(value);
        self
    }
    pub const fn build(self)->RegisterU16{
        self.reg
    }
}
pub trait RegisterSelection<const N:u8>{
    type RegType;
}
impl RegisterSelection<0> for (){
    type RegType = Register;
}
impl RegisterSelection<1> for (){
    type RegType = RegisterU16;
}
impl RegisterSelection<2> for (){
    type RegType = Register;
}



