pub struct CallBacks<const N:usize, F>{
    call_array:[Option<F>;N],
}
impl<const N:usize,F:Copy> CallBacks<N,F>{
    pub const fn new()->Self{
        Self{ call_array:[None;N], }
    }
    pub fn set_callback(&mut self,n:usize, f:F){
        self.call_array[n].replace(f);
    }
    pub fn call(&self,n:usize)->Option<F>{
        self.call_array[n]
    }
}
