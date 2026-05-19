use crate::mcu::registers::{eicra::*, eifr::EifrBuilder, eimsk::*};
use crate::mcu::registers::{tccr0a::*,::tccr0b::*};
use crate::mcu::registers::{tccr1a::*,::tccr1b::*};
use crate::mcu::registers::{tccr2a::*,::tccr2b::*};
pub trait Bitsable{
    fn set_mask(&mut self, mask:u8);
    fn reset_mask(&mut self, mask:u8);
    fn toggle_mask(&mut self, mask:u8);
}
macro_rules! impl_bits_able {
        ($($t:ty),*) => {
        $(
            impl Bitsable for $t {
                fn set_mask(&mut self, mask: u8) { self.0 |= mask; }
                fn reset_mask(&mut self, mask: u8) { self.0 &= !mask; }
                fn toggle_mask(&mut self, mask: u8) { self.0 ^= mask; }
            }
        )*
    };
}
impl_bits_able!(EicraBuilder,EimskBuilder,EifrBuilder);
impl_bits_able!(Tccr0aBuilder,Tccr0bBuilder);
impl_bits_able!(Tccr1aBuilder,Tccr1bBuilder);
impl_bits_able!(Tccr2aBuilder,Tccr2bBuilder);
impl_bits_able!(Timsk0Builder,Timsk1Builder, Timsk2Builder);
