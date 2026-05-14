#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]
use core::option_env;
pub const F_CPU: u32 = parse_u32(
    match option_env!("F_CPU") {
        Some(val) => val,
        None => "16000000", // Дефолт, если build.rs не отработал
    }
);

const fn parse_u32(s: &str) -> u32 {
    let b = s.as_bytes();
    let mut res = 0u32;
    let mut i = 0;
    while i < b.len() {
        res = res * 10 + (b[i] - b'0') as u32; // 0x30 это код символа '0'
        i += 1;
    }
    res
}

pub mod drivers;
pub mod mcu;
pub mod utils;

pub mod interrupts;
pub mod panics;
