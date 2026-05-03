use super::*;
// #[repr(u8)]
// pub enum Ucsr0aBits{
//     MPCM0 = 0,
//     U2X0  = 1,
//     UPE0  = 2,
//     DOR0  = 3,
//     FE0   = 4,
//     UDRE0 = 5,
//     TXC0  = 6,
//     RXC0  = 7,
// }
pub struct Ucsr0aBuilder(pub u8);
impl Ucsr0aBuilder{
    pub fn en_mpcm0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::MPCM0 as u8;
        self
    }    
    pub fn dis_mpcm0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::MPCM0 as u8);
        self
    }    
    pub fn en_u2x0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::U2X0 as u8;
        self
    }    
    pub fn dis_u2x0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::U2X0 as u8);
        self
    }    
    pub fn en_upe0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::UPE0 as u8;
        self
    }    
    pub fn dis_upe0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::UPE0 as u8);
        self
    }    
    pub fn en_dor0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::DOR0 as u8;
        self
    }    
    pub fn dis_dor0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::DOR0 as u8);
        self
    }    
    pub fn en_fe0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::FE0 as u8;
        self
    }    
    pub fn dis_fe0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::FE0 as u8);
        self
    }    
    pub fn en_udre0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::UDRE0 as u8;
        self
    }    
    pub fn dis_udre0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::UDRE0 as u8);
        self
    }    
    pub fn en_txc0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::TXC0 as u8;
        self
    }    
    pub fn dis_txc0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::TXC0 as u8);
        self
    }    
    pub fn en_rxc0(mut self)->Self{
        self.0 |= 1 << Ucsr0aBits::RXC0 as u8;
        self
    }    
    pub fn dis_rxc0(mut self)->Self{
        self.0 &= !(1 << Ucsr0aBits::RXC0 as u8);
        self
    }    
}
//----------------------------------------------//
pub struct Ucsr0bBuilder ( pub u8);
impl Ucsr0bBuilder{
    pub fn en_txci(mut self)->Self{
        self.0 |= 1<<Ucsr0bBits::TXCIE0 as u8;
        self
    }
    pub fn dis_txci(mut self)->Self{
        self.0 &= !1<<Ucsr0bBits::TXCIE0 as u8;
        self
    }
    pub fn en_tx(mut self)->Self{
        self.0 |= 1<<Ucsr0bBits::TXEN0 as u8;
        self
    }
    pub fn dis_tx(mut self)->Self{
        self.0 &= !1<<Ucsr0bBits::TXEN0 as u8;
        self
    }
    pub fn en_rx(mut self)->Self{
        self.0 |= 1<<Ucsr0bBits::RXEN0 as u8;
        self
    }
    pub fn dis_rx(mut self)->Self{
        self.0 &= !1<<Ucsr0bBits::RXEN0 as u8;
        self
    }
    pub fn en_rxci(mut self)->Self{
        self.0 |= 1<<Ucsr0bBits::RXCIE0 as u8;
        self
    }
    pub fn dis_rxci(mut self)->Self{
        self.0 &= !1<<Ucsr0bBits::RXCIE0 as u8;
        self
    }
    pub fn build(&self)->u8{
        self.0
    }
}
//----------------------------------------------//
pub struct Ucsr0cBuilder(pub u8);
impl Ucsr0cBuilder{
    pub fn en_ucpol0(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UCPOL0 as u8;
        self
    }
    pub fn dis_ucpol0(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UCPOL0 as u8);
        self
    }
    pub fn en_ucsz00(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UCSZ00 as u8;
        self
    }
    pub fn dis_ucsz00(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UCSZ00 as u8);
        self
    }
    pub fn en_ucsz01(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UCSZ01 as u8;
        self
    }
    pub  fn dis_ucsz01(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UCSZ01 as u8);
        self
    }
    pub fn en_usbs0(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UCSZ01 as u8;
        self
    }
    pub fn dis_usbs0(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UCSZ01 as u8);
        self
    }
    pub fn en_upm00(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UPM00 as u8;
        self
    }
    pub fn dis_upm00(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UPM00 as u8);
        self
    }
    pub fn en_upm01(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UPM01 as u8;
        self
    }
    pub fn dis_upm01(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UPM01 as u8);
        self
    }
    pub fn en_umsel00(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UMSEL00 as u8;
        self
    }
    pub fn dis_umsel00(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UMSEL00 as u8);
        self
    }
    pub fn en_umsel01(mut self)->Self{
        self.0 |= 1 << Ucsr0cBits::UMSEL01 as u8;
        self
    }
    pub fn dis_umsel01(mut self)->Self{
        self.0 &= !(1 << Ucsr0cBits::UMSEL01 as u8);
        self
    }
    pub fn build(&self)->u8{ self.0 }
}

