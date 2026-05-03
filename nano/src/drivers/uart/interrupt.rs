use crate::utils::{register::*, callable::*};
use super::*;
use super::builders::Ucsr0bBuilder;
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
            true => self.ucsr0b.modify( |v| v|Ucsr0bBuilder(0).en_rx().en_rxci().build() ),
            false => self.ucsr0b.modify( |v| v|Ucsr0bBuilder(0).dis_rx().dis_rxci().build() ),
        }
    }
    pub fn enable_txi(&mut self, value:bool ) {
        match value{
            true => self.ucsr0b.modify(|v| v|Ucsr0bBuilder(0).en_tx().en_txci().build()),
            false => self.ucsr0b.modify(|v| v|Ucsr0bBuilder(0).dis_tx().dis_txci().build()),
        }
    }
}
