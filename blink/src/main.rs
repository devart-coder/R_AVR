#![no_std]
#![no_main]
use nano::drivers::timers::timer::*;
use nano::drivers::timers::action::*;
use nano::drivers::timers::interrupt::*;
use nano::mcu::pins::Pins;
use nano::interrupts::*;
use nano::utils::delay::*;
#[unsafe(no_mangle)]
fn main() -> ! {
    sei();
    Pins::take().d6.into_output();
    let t0 = Timer::<0>::get();
    t0.interrupt().a_channel_enable(true);
    t0.interrupt().b_channel_enable(true);
    t0.callback().channal_a( 
        || {
            let p = Pins::take().d13.into_output();
            p.set_high();
        }
    );
    t0.callback().channal_b(
        || {
            let p = Pins::take().d13.into_output();
            p.set_low();
        }
    );
    t0.settings().default();
    loop{
        for i in 0..=255{
            t0.action().set_counter_a(i);
            delay_ms(2);
        }
        for i in 0..=255{
            t0.action().set_counter_b(i);
            delay_ms(2);
        }
    };
}
