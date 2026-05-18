use super::bitsable::Bitsable;

pub struct BitOperations<const BIT:u8,T:Bitsable> (pub T);
impl<const BIT:u8, T:Bitsable> BitOperations<BIT, T>{
    const MASK:u8 = 1<<BIT;
    pub fn set_bit(mut self)->T{
        self.0.set_mask(Self::MASK);
        self.0
    }
    pub fn clear_bit(mut self)->T{
        self.0.reset_mask(Self::MASK);
        self.0
    }
    pub fn toogle(mut self) -> T{
        self.0.toggle_mask(Self::MASK);
        self.0
    }
}