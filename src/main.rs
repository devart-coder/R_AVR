#![no_std]    // 1. Отключаем стандартную библиотеку (её нет на Arduino)
#![no_main]   // 2. Указываем, что стандартной "точки входа" нет

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



use no::register::Register;
#[unsafe(no_mangle)]
fn main() -> ! {
    loop {
        // Тут будет ваш код работы с регистрами
    }
}
