pub struct Register{
    pub address:*mut u8
}
impl Register {
    pub fn read(&self)->u8{
        unsafe{
            core::ptr::read_volatile(self.address)
        }
    }
    pub fn write(&mut self, value:u8){
        unsafe{
            core::ptr::write_volatile(self.address,value);
        }
    }
}
