use core::arch::asm;
use super::drivers::uart::*;
use crate::drivers::timers::timer::*;
pub fn sei(){
    unsafe{ asm!("sei"); }
}

pub fn cli(){
    unsafe{ asm!("cli"); }
}
#[unsafe(no_mangle)]

pub extern "avr-interrupt" fn __vector_18(){//UART_RX_COMPLETE
    uart.interrupt.handle_rx();
}
pub extern "avr-interrupt" fn __vector_19(){//UART_DRE
}
pub extern "avr-interrupt" fn __vector_20(){//UART_TX_COMPLETE
    uart.interrupt.handle_tx();
}
//---TIMER_2---//
pub extern "avr-interrupt" fn __vector_7(){//TIMER2_COMPA
    TIMER_2.interrupt.channal_a_callback();
}
pub extern "avr-interrupt" fn __vector_8(){//TIMER2_COMPB
    TIMER_2.interrupt.channal_b_callback();
}
pub extern "avr-interrupt" fn __vector_9(){//TIMER2_OVF
    TIMER_2.interrupt.ovf_callback();
}
//---TIMER_1---//
pub extern "avr-interrupt" fn __vector_10(){//TIMER1_CAPT
    TIMER_1.interrupt.channal_a_callback();
}
pub extern "avr-interrupt" fn __vector_11(){//TIMER1_COMPA
    TIMER_1.interrupt.channal_b_callback();

}
pub extern "avr-interrupt" fn __vector_12(){//TIMER1_COMPB
}
pub extern "avr-interrupt" fn __vector_13(){//TIMER1_OVF

}
//---TIMER_0---//
pub extern "avr-interrupt" fn __vector_14(){//TIMER0_COMPA
    TIMER_0.interrupt.channal_a_callback();
}
pub extern "avr-interrupt" fn __vector_15(){//TIMER0_COMPB
    TIMER_0.interrupt.channal_b_callback();

}
pub extern "avr-interrupt" fn __vector_16(){//TIMER0_OVF
    TIMER_0.interrupt.ovf_callback();
}
