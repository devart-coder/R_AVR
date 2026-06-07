use nano::drivers::twi::{
    master::Master,
    twi::{PrescalerMode, Twi},
};

pub struct Bmp180 {
    twi: Master,
}
impl Bmp180 {
    pub fn new(mode: PrescalerMode, speed: u8) -> Self {
        Self {
            twi: Twi::new().prescaler_mode(mode).speed(speed).as_master(),
        }
    }
}
