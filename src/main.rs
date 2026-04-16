#![no_std]
#![no_main]
use nano::uart::*;
#[unsafe(no_mangle)]
fn main() -> ! {
    uart.settings.default();
    loop{
    }
}
