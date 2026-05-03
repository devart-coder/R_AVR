use crate::utils::{register::*, callable::*};
use super::*;
pub struct Interrupt{
    ucsr0b: Register,
    call_array : CallBacks<2,fn()>,
}
impl Interrupt{
    pub const fn new()->Self{
        Interrupt {
            ucsr0b: Register::new(UartRegisters::UCSR0B as u8),
            call_array : CallBacks::<2,fn()>::new(),
        }
    }
    pub fn set_rx_handle(&mut self, f:fn()){
        self.call_array.set_callback(0,f);
    }
    pub fn set_tx_handle(&mut self, f:fn()){
        self.call_array.set_callback(1,f);
    }
    pub fn handle_rx(&self) -> Option<fn()>{
        self.call_array.call(0)
    }
    pub fn handle_tx(&self) -> Option<fn()>{
        self.call_array.call(1)
    }
    pub fn enable_rxi(&mut self, value:bool ) {
        match value{
            true => self.modify( |v| v.en_rx().en_rxci() ),
            false => self.modify( |v| v.dis_rx().dis_rxci() ),
        }
    }
    pub fn enable_txi(&mut self, value:bool ) {
        match value{
            true => self.modify( |v| v.en_tx().en_txci() ),
            false => self.modify( |v| v.dis_tx().dis_txci() ),
        }
    }
    pub fn modify<T>(&mut self, value:T) where T:FnOnce(Ucsr0bBuilder)->Ucsr0bBuilder{
        let v = value(Ucsr0bBuilder::new()).build();
        self.ucsr0b.modify(|reg| reg|v );
    }
}

pub struct Ucsr0bBuilder { value:u8, }

impl Ucsr0bBuilder{
    fn new()->Self{
        Self{value:0}
    }
    fn en_txci(mut self)->Self{
        self.value |= 1<<Ucsr0bBits::TXCIE0 as u8;
        self
    }
    fn dis_txci(mut self)->Self{
        self.value &= !1<<Ucsr0bBits::TXCIE0 as u8;
        self
    }
    fn en_tx(mut self)->Self{
        self.value |= 1<<Ucsr0bBits::TXEN0 as u8;
        self
    }
    fn dis_tx(mut self)->Self{
        self.value &= !1<<Ucsr0bBits::TXEN0 as u8;
        self
    }
    fn en_rx(mut self)->Self{
        self.value |= 1<<Ucsr0bBits::RXEN0 as u8;
        self
    }
    fn dis_rx(mut self)->Self{
        self.value &= !1<<Ucsr0bBits::RXEN0 as u8;
        self
    }
    fn en_rxci(mut self)->Self{
        self.value |= 1<<Ucsr0bBits::RXCIE0 as u8;
        self
    }
    fn dis_rxci(mut self)->Self{
        self.value &= !1<<Ucsr0bBits::RXCIE0 as u8;
        self
    }
    fn build(&self)->u8{
        self.value
    }
}
