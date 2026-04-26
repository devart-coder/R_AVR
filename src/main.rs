#![no_std]
#![no_main]
use nano::drivers::timers::timer::*;
#[unsafe(no_mangle)]
fn main() -> ! {
    TIMER_0.action;
    loop{
    }
}
