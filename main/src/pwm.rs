#![no_std]
#![no_main]
use nano::drivers::timers::settings::Prescaling;
use nano::drivers::*;
use nano::mcu::pins::Pins;
use nano::delay::Delay;
#[unsafe(no_mangle)]
fn main(){
    let pins = Pins::take();
    pins.d5.into_output();//blue
    pins.d6.into_output();//red
    let mut pwm = Timer::<0>::get().into_pwm();
    pwm.prescaling(Prescaling::_1024);
    let (mut a,mut b) = pwm.channels();
    a.set_mode(PwmMode::NonInverted);
    b.set_mode(PwmMode::NonInverted);
    loop{
        for i in 0..=255{
            a.set_duty(i);
            Delay::delay_ms(1);
        }
        for i in (0..=255).rev(){
            a.set_duty(i);
            Delay::delay_ms(1);
        }
        for i in 0..=255{
            b.set_duty(i);
            Delay::delay_ms(1);
        }
        for i in (0..=255).rev(){
            b.set_duty(i);
            Delay::delay_ms(1);
        }
    }
}
