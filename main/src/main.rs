#![no_std]
#![no_main]
use nano::drivers::uart::{interrupt::*, uart::uart};
#[unsafe(no_mangle)]
fn main(){
    let u= uart;
    uart.interrupt;
    u.output.send_slice("string");
}
