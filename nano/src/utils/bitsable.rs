use crate::mcu::registers::{eicra::*,eimsk::*};
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
impl_bits_able!(EicraBuilder,EimskBuilder);
