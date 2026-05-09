#![no_std]
#![no_main]
use nano::drivers::Pwm;
use nano::drivers::Timer;
#[unsafe(no_mangle)]
fn main(){
    let _p = Pwm::new(Timer::<0>::get());
    loop{
    }
}
