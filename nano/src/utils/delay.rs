use crate::F_CPU;
use core::arch::asm;
pub const fn ms_const_value() -> u16 {
    match F_CPU {
        16_000_000 => 4000,
        //ToDo::Make fo 1_000_000Mhz
        _ => 0,
    }
}
pub fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..=ms_const_value() {
            unsafe { asm!("nop") };
        }
    }
}
pub fn delay_us(us: u16) {
    for _ in 0..=us {
        unsafe {
            asm!(
                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop"
            )
        };
    }
}
