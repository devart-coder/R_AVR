use core::arch::asm;
use super::drivers::uart::uart::Uart;
use crate::drivers::timers::timer::*;
pub fn sei(){ unsafe{ asm!("sei"); } }
pub fn cli(){ unsafe{ asm!("cli"); } }

#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_18(){//UART_RX_COMPLETE
    Uart::get().interrupt.handle_rx();
}
#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_19(){//UART_DRE
}
#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_20(){//UART_TX_COMPLETE
    Uart::get().interrupt.handle_tx();
}
//---TIMER_2---//
// pub extern "avr-interrupt" fn __vector_7(){//TIMER2_COMPA
//     TIMER_2.callback.channal_a_callback().unwrap();
// }
// pub extern "avr-interrupt" fn __vector_8(){//TIMER2_COMPB
//     TIMER_2.callback.channal_b_callback();
// }
// pub extern "avr-interrupt" fn __vector_9(){//TIMER2_OVF
//     TIMER_2.callback.ovf_callback();
// }
// //---TIMER_1---//
// pub extern "avr-interrupt" fn __vector_10(){//TIMER1_CAPT
//     TIMER_1.callback.channal_a_callback();
// }
// pub extern "avr-interrupt" fn __vector_11(){//TIMER1_COMPA
//     TIMER_1.callback.channal_b_callback();

// }
pub extern "avr-interrupt" fn __vector_12(){//TIMER1_COMPB
}
pub extern "avr-interrupt" fn __vector_13(){//TIMER1_OVF

}
#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_14(){//TIMER0_COMPA
    if let Some(f) = Timer::<0>::get().callback().channal_a_callback(){
        f();
    }
}
#[unsafe(no_mangle)]
pub extern "avr-interrupt" fn __vector_15(){//TIMER0_COMPB
    if let Some(f) = Timer::<0>::get().callback().channal_b_callback(){
            f();
    }
}
// pub extern "avr-interrupt" fn __vector_16(){//TIMER0_OVF
//     // TIMER_0.callback.ovf_callback().unwrap()();
// }
