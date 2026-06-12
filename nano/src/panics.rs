use core::panic::PanicInfo;

use crate::mcu::pins::Pins;
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {
        Pins::take().d13.into_output().set_high();
    }
}
