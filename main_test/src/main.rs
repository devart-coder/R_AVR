#![no_std]
#![no_main]
use nano::drivers::timers::settings::Prescaling;
use nano::drivers::*;
use nano::utils::delay::*;
#[unsafe(no_mangle)]
fn main(){
    let mut pwm = Timer::<1>::get().into_pwm();
    pwm.prescaling(Prescaling::_64);
    
    let (mut a,mut b) = pwm.channels();
    a.set_mode(PwmMode::NonInverted);
    a.get_pin().into_output();
    
    b.set_mode(PwmMode::NonInverted);
    b.get_pin().into_output();
    loop{
        for i in 0..=255{
            a.set_duty(i);
            delay_ms(1);
        }
        for i in (0..=255).rev(){
            a.set_duty(i);
            delay_ms(1);
        }
        for i in 0..=255{
            b.set_duty(i);
            delay_ms(1);
        }
        for i in (0..=255).rev(){
            b.set_duty(i);
            delay_ms(1);
        }
    }
}
