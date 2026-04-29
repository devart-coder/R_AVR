#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]
use nano::drivers::timers::timer::*;
use nano::drivers::timers::action::*;
use nano::drivers::timers::settings::*;
use nano::drivers::timers::interrupt::*;
use nano::mcu::pins::Pins;
use nano::interrupts::*;
use nano::delay::Delay;
    // const timer:Timer<0> = Timer::<0>::new();
#[unsafe(no_mangle)]
fn main() -> ! {
    sei();
    Pins::take().d6.into_output();
    let t0 = Timer::<0>::get();
    t0.interrupt.a_channel_enable(true);
    t0.interrupt.b_channel_enable(true);
    t0.settings.set_mode(Mode::Normal);
    unsafe{
        (*t0.callback).channal_a(
        ||{
            let p = Pins::take().d13.into_output();
            p.set_high();
        });
    };
    t0.settings.prescaling(Prescaling::_64);
    loop{
        for i in 0..=255{
            unsafe{t0.action.set_counter_a(i);};
            Delay::delay_ms(1);
        }
        for i in 0..=255{
            unsafe{t0.action.set_counter_b(i);};
            Delay::delay_ms(1);
        }
    };
}


fn f(){
        let p = Pins::take().d13.into_output();
        p.set_high();
}
#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_15(){//TIMER0_COMPB
    let p = Pins::take().d13.into_output();
    p.set_low();
}
