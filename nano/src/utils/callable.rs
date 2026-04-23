pub struct CallBacks<const N:usize,Arg>{
    call_array:[Option<fn(Arg)>;N],
}
impl<const N:usize, Arg> CallBacks<N,Arg>{
    pub const fn new()->Self{
        Self{ call_array:[None;N], }
    }
    pub fn set_callback(&mut self,n:usize, f:fn(Arg)){
        self.call_array[n].replace(f);
    }
    pub fn call(&self,n:usize)->Option<fn(Arg)>{
        self.call_array[n]
    }
}
