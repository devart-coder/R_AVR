use crate::utils::bit_operations::BitOperations;

pub enum TwsrBits{//TWIStatusRegister
    Twps0 = 0,
    Twps1 = 1,
    Tws3  = 3,
    Tws4  = 4,
    Tws5  = 5,
    Tws6  = 6,
    Tws7  = 7,
}
impl TwsrBits{
    const TWPS0:u8 = Self::Twps0 as u8;
    const TWPS1:u8 = Self::Twps1 as u8;
    const TWS3:u8  = Self::Tws3  as u8;
    const TWS4:u8  = Self::Tws4  as u8;
    const TWS5:u8  = Self::Tws5  as u8;
    const TWS6:u8  = Self::Tws6  as u8;
    const TWS7:u8  = Self::Tws7  as u8;
}

pub struct TwsrBuilder (pub u8);
impl TwsrBuilder {
    pub fn twps0(self)->BitOperations<{TwsrBits::TWPS0},Self>{
        BitOperations::<{TwsrBits::TWPS0},Self>(self)
    }   
    pub fn twps1(self)->BitOperations<{TwsrBits::TWPS1},Self>{
        BitOperations::<{TwsrBits::TWPS1},Self>(self)
    }
    pub fn tws3(self)->BitOperations<{TwsrBits::TWS3},Self>{
        BitOperations::<{TwsrBits::TWS3},Self>(self)
    }
    pub fn tws4(self)->BitOperations<{TwsrBits::TWS4},Self>{
        BitOperations::<{TwsrBits::TWS4},Self>(self)
    }
    pub fn tws5(self)->BitOperations<{TwsrBits::TWS5},Self>{
        BitOperations::<{TwsrBits::TWS5},Self>(self)
    }
    pub fn tws6(self)->BitOperations<{TwsrBits::TWS6},Self>{
        BitOperations::<{TwsrBits::TWS6},Self>(self)
    }
    pub fn tws7(self)->BitOperations<{TwsrBits::TWS7},Self>{
        BitOperations::<{TwsrBits::TWS7},Self>(self)
    }
}