pub struct Register<T>{
    address:*mut T
}
impl Register<u8> {
    pub const fn new(address: usize)->Self{
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
