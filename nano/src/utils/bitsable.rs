use crate::mcu::registers::{eicra::EicraBuilder, eifr::EifrBuilder, eimsk::EimskBuilder};
use crate::mcu::registers::{tccr0a::Tccr0aBuilder,tccr0b::Tccr0bBuilder};
use crate::mcu::registers::{tccr1a::Tccr1aBuilder,tccr1b::Tccr1bBuilder};
use crate::mcu::registers::{tccr2a::Tccr2aBuilder,tccr2b::Tccr2bBuilder};
use crate::mcu::registers::timsk0::Timsk0Builder;
use crate::mcu::registers::timsk1::Timsk1Builder;
use crate::mcu::registers::timsk2::Timsk2Builder;
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
impl_bits_able!(Timsk0Builder,Timsk1Builder,Timsk2Builder);
