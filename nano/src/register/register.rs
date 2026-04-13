pub struct Register{
    address:*mut u8
}
impl Register {
    pub fn read(&self)->u8{
        unsafe{
            core::prt::read_volatile(self.address)
        }
    }
    pub fn write(&mut self, value:u8){
        unsafe{
            core::prt::write_volatile(self.address,value);
        }
    }
}
