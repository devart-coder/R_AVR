use core::arch::asm;
pub struct Delay{
}
impl Delay{
    pub const fn ms_const_value()->u16{ 4000 }
	pub fn delay_ms(ms:u16){
	    for _ in 0..ms{
		    for _ in 0..Delay::ms_const_value(){
			    unsafe{asm!("nop")};
			}
		}
	}
	pub fn delay_us(us:u16){
	    for _ in 0..us{
		    unsafe{
			    asm!(
				    "nop","nop","nop","nop","nop",
					"nop","nop","nop","nop","nop"
				)
			};
		}
	}
}
