#![no_std]
#![no_main]
use core::str::from_utf8;
use nano::drivers::uart::*;
use nano::mcu::pins::*;

#[unsafe(no_mangle)]
fn main() -> ! {
    uart.settings.default();
    uart.output.send_slice("Hello world!\n");
    let led = Pins::take().d13.into_output();
    loop{
        let c = uart.input.receive_byte_slice();
        let r = from_utf8(&c).unwrap();

        uart.output.send_slice(r);
        if r == "2" {
            led.set_high();
        }else if r == "1"{
            led.set_low();
        }
    }
}
