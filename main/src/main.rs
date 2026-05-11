#![no_std]
#![no_main]
use nano::drivers::timers::settings::Prescaling;
use nano::drivers::*;
use nano::drivers::PwmMode;
use nano::drivers::Pwmable;
use nano::drivers::Timer;
use nano::mcu::pins::Pins;
use nano::delay::Delay;
#[unsafe(no_mangle)]
fn main(){
    let pins = Pins::take();
    pins.d5.into_output();
    pins.d6.into_output();
    let mut t =Timer::<0>::get().into_pwm();
    t.prescaling(Prescaling::_64);
    // let mut a = t.a_channel();
    let mut b = t.b_channel();
    // a.set_mode(PwmMode::NonInverted);
    b.set_mode(PwmMode::NonInverted);
    loop{
        for i in 0..=255{
            // a.set_duty(i);
            Delay::delay_ms(1);
        }
        for i in (0..=255).rev(){
            b.set_duty(i);
            Delay::delay_ms(1);
        }
    }
}
