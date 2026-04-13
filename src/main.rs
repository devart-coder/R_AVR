#![no_std]
#![no_main]

fn main() -> !{
    loop{

    }
}
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // Здесь можно мигать светодиодом ошибки, если есть
    }
}
