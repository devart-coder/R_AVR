#![no_std]
#![no_main]
use nano::Uart;
use nano::delay::Delay;
#[unsafe(no_mangle)]
fn main(){
    let mut u= Uart::new();
    u.settings.default();
    loop{
        u.output.send_slice("string\n");
        Delay::delay_ms(1000/2);
    }
}
