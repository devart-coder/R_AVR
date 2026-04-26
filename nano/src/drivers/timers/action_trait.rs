use super::action::Action;
pub trait ActionTrait{
    type ArgType;
    fn set_counter_a(&mut self, value:Self::ArgType);
    fn set_counter_b(&mut self, value:Self::ArgType);
    fn set_timer_counter(&mut self, value:Self::ArgType);
    fn counter_a(&self)->Self::ArgType;
    fn counter_b(&self)->Self::ArgType;
    fn timer_counter(&self)->Self::ArgType;
}
impl ActionTrait for Action<0>{
    type ArgType = u8;
    fn set_counter_a(&mut self, value:Self::ArgType){
        self.ocra.write(value);
    }
    fn set_counter_b(&mut self, value:Self::ArgType){
        self.ocrb.write(value);
    }
    fn set_timer_counter(&mut self, value:Self::ArgType){
        self.tcnt.write(value);
    }
    fn counter_a(&self)->Self::ArgType{
        self.ocra.read()
    }
    fn counter_b(&self)->Self::ArgType{
        self.ocrb.read()
    }
    fn timer_counter(&self)->Self::ArgType{
        self.tcnt.read()
    }
}
impl ActionTrait for Action<1>{
    type ArgType = u16;
    fn set_counter_a(&mut self, value:Self::ArgType){
        self.ocra.write(value);
    }
    fn set_counter_b(&mut self, value:Self::ArgType){
        self.ocrb.write(value);
    }
    fn set_timer_counter(&mut self, value:Self::ArgType){
    }
    fn counter_a(&self)->Self::ArgType{
        self.ocra.read()
    }
    fn counter_b(&self)->Self::ArgType{
        self.ocrb.read()
    }
    fn timer_counter(&self)->Self::ArgType{
        self.tcnt.read()
    }
}


