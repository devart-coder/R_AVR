use crate::mcu::registers::*;
use crate::utils::register::*;
use crate::interrupts::sei;
pub enum Prescaling {
    NoSource = 0,
    _1 = 1,
    _8 = 2,
    _64 = 3,
    _256 = 4,
    _1024 = 5,
    ExternalClockFalling = 6,
    ExternalClockRising = 7,
}
pub enum Mode {
    Normal = 0,
    CTC = 1,
    PwdFast = 2,
    PwdPhaseCorrect = 3,
}
pub struct Settings<const N:u8>{
    _prescaling:Prescaling,
    _tccrb:Register,
    _tccra:Register,
}
impl<const N:u8> Settings<N>{
    pub fn tccra(&mut self)->&mut Register{
        &mut self._tccra
    }
    pub fn tccrab(&mut self)->&mut Register{
        &mut self._tccrb
    }
    pub const fn new() -> Option<Self>{
        match N{
            0 =>Some(
                Settings{
                    _tccra:Register::new(TimerRegisters::TCCR0A as u8),
                    _tccrb:Register::new(TimerRegisters::TCCR0B as u8),
                    _prescaling: Prescaling::NoSource,
                }
            ),
            1 =>Some(
                Settings{
                    _tccra:Register::new(TimerRegisters::TCCR1A as u8),
                    _tccrb:Register::new(TimerRegisters::TCCR1B as u8),
                    _prescaling: Prescaling::NoSource,
                }
            ),
            2 =>Some(
                Settings{
                    _tccra:Register::new(TimerRegisters::TCCR2A as u8),
                    _tccrb:Register::new(TimerRegisters::TCCR2B as u8),
                    _prescaling: Prescaling::NoSource,
                }
            ),
            _ =>None,
        }
    }
    pub fn prescaling(&mut self, p:Prescaling){
        let value:u8 = match p {
            Prescaling::NoSource =>{
                match N {
                    0=> !((1<<Tccr0bBits::CS02 as u8)|(1<<Tccr0bBits::CS01 as u8)|(1<<Tccr0bBits::CS00 as u8)),
                    1=> !((1<<Tccr1bBits::CS12 as u8)|(1<<Tccr1bBits::CS11 as u8)|(1<<Tccr1bBits::CS10 as u8)),
                    2=> !((1<<Tccr2bBits::CS22 as u8)|(1<<Tccr2bBits::CS21 as u8)|(1<<Tccr2bBits::CS20 as u8)),
                    _=> 0,
                }
            },
            Prescaling::_1 =>{
                match N {
                    0=> 1<<Tccr0bBits::CS00 as u8,
                    1=> 1<<Tccr1bBits::CS10 as u8,
                    2=> 1<<Tccr2bBits::CS20 as u8,
                    _=> 0,
                }
            },
            Prescaling::_8 =>{
                match N {
                    0=> 1<<Tccr0bBits::CS01 as u8,
                    1=> 1<<Tccr1bBits::CS11 as u8,
                    2=> 1<<Tccr2bBits::CS21 as u8,
                    _=> 0,
                }
            },
            Prescaling::_64 =>{
                match N {
                    0=> (1<<Tccr0bBits::CS01 as u8)|(1<<Tccr0bBits::CS00 as u8),
                    1=> (1<<Tccr1bBits::CS11 as u8)|(1<<Tccr1bBits::CS10 as u8),
                    2=> (1<<Tccr2bBits::CS21 as u8)|(1<<Tccr2bBits::CS20 as u8),
                    _=> 0,
                }
            },
            Prescaling::_256=>{
                match N {
                    0=> 1<<Tccr0bBits::CS02 as u8,
                    1=> 1<<Tccr1bBits::CS12 as u8,
                    2=> 1<<Tccr2bBits::CS22 as u8,
                    _=> 0,
                }
            },
            Prescaling::_1024=>{
                match N {
                    0=> (1<<Tccr0bBits::CS02 as u8)|(1<<Tccr0bBits::CS00 as u8),
                    1=> (1<<Tccr1bBits::CS12 as u8)|(1<<Tccr1bBits::CS10 as u8),
                    2=> (1<<Tccr2bBits::CS22 as u8)|(1<<Tccr2bBits::CS20 as u8),
                    _=> 0,
                }
            },
            //TODO::applyImpl
            Prescaling::ExternalClockFalling=>{
                match N{
                    _=> 0u8,
                }
            },
            //TODO::applyImpl
            Prescaling::ExternalClockRising=>{
                match N{
                    _=> 0u8,
                }
            },
        };
        if let Some(Prescaling::NoSource) = Some(&p) {
            self._tccrb.modify(|w| w&value);
        }else{
            self._tccrb.modify(|w| w|value);
        }
        self._prescaling = p;
    }
    pub fn set_mode(&self,mode:Mode){
        let n:u8 = match mode{
            Mode::Normal=>{
                match N{
                    0=>!((1<<Tccr0aBits::WGM00 as u8)|(1<<Tccr0aBits::WGM01 as u8)),
                    1=>!((1<<Tccr1aBits::WGM10 as u8)|(1<<Tccr1aBits::WGM11 as u8)),
                    2=>!((1<<Tccr2aBits::WGM20 as u8)|(1<<Tccr2aBits::WGM21 as u8)),
                    _=>0,
                }
            },
            Mode::CTC=>{
                match N{
                    0=>1<<Tccr0aBits::WGM01 as u8,
                    1=>1<<Tccr1aBits::WGM11 as u8,
                    2=>1<<Tccr2aBits::WGM21 as u8,
                    _=>0,
                }
            },
            Mode::PwdFast=>{
                match N{
                    0=>(1<<Tccr0aBits::WGM00 as u8)|(1<<Tccr0aBits::WGM01 as u8),
                    1=>(1<<Tccr1aBits::WGM10 as u8)|(1<<Tccr1aBits::WGM11 as u8),
                    2=>(1<<Tccr2aBits::WGM20 as u8)|(1<<Tccr2aBits::WGM21 as u8),
                    _=>0,
                }
            },
            Mode::PwdPhaseCorrect=>{
                match N{
                    0=>1<<Tccr0aBits::WGM00 as u8,
                    1=>1<<Tccr1aBits::WGM10 as u8,
                    2=>1<<Tccr2aBits::WGM20 as u8,
                    _=>0,
                }
            },
        };
        self._tccra.modify(|w| w|n);
    }
    pub fn default(&mut self){
        self.prescaling(Prescaling::_64);
        self.set_mode(Mode::Normal);
        sei();
    }
}
impl<const N:u8> Clone for Settings<N>{
    fn clone(&self) -> Self {
        Settings::<N>::new().unwrap()
    }
}
