use super::timer::Timer;
pub trait TimerTrait{
    fn overflow_counter(&self, count:u8);
    // fn set_chanel();
    // fn channel_callback(&self, symbol:&str);
}
impl TimerTrait for Timer<0>{
    fn overflow_counter(&self, count:u8){
    }
    // fn chanel_overflow(&self, count:u8){
    // }
}
impl TimerTrait for Timer<2>{
    fn overflow_counter(&self, count:u8){
    }
    // fn chanel_overflow(&self, count:u8){
    // }
}
