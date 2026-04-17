use core::arch::asm;

pub fn sei(){
    unsafe{
        asm!("sei");
    }
}

pub fn cli(){
    unsafe{
        asm!("cli");
    }
}
#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_18(){//UART_RX_COMPLETE

}
pub extern "avr-interrupt" fn __vector_19(){//UART_DRE

}
pub extern "avr-interrupt" fn __vector_20(){//UART_TX_COMPLETE

}
