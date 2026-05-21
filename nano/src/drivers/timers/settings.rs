use crate::mcu::registers::tccr0a::Tccr0aBuilder;
use crate::mcu::registers::tccr0b::{Tccr0bBits, Tccr0bBuilder};
use crate::mcu::registers::tccr1a::Tccr1aBuilder;
use crate::mcu::registers::tccr1b::{Tccr1bBits, Tccr1bBuilder};
use crate::mcu::registers::tccr2a::Tccr2aBuilder;
use crate::mcu::registers::tccr2b::Tccr2bBuilder;
use crate::mcu::registers_list::*;
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
                    0=> Tccr0bBuilder(0).build(),
                    1=> Tccr1bBuilder(0).build(),
                    2=> Tccr2bBuilder(0).build(),
                    _=> 0,
                }
            },
            Prescaling::_1 =>{
                match N {
                    0=> Tccr0bBuilder(0).cs00().set_bit().build(),
                    1=> Tccr1bBuilder(0).cs10().set_bit().build(),
                    2=> Tccr2bBuilder(0).cs20().set_bit().build(),
                    _=> 0,
                }
            },
            Prescaling::_8 =>{
                match N {
                    0=> Tccr0bBuilder(0).cs01().set_bit().build(),
                    1=> Tccr1bBuilder(0).cs11().set_bit().build(),
                    2=> Tccr2bBuilder(0).cs21().set_bit().build(),
                    _=> 0,
                }
            },
            Prescaling::_64 =>{
                match N {
                    0=> Tccr0bBuilder(0).cs00().set_bit().cs01().set_bit().build(),
                    1=> Tccr1bBuilder(0).cs10().set_bit().cs11().set_bit().build(),
                    2=> Tccr2bBuilder(0).cs20().set_bit().cs21().set_bit().build(),
                    _=> 0,
                }
            },
            Prescaling::_256=>{
                match N {
                    0=> Tccr0bBuilder(0).cs02().set_bit().build(),
                    1=> Tccr1bBuilder(0).cs12().set_bit().build(),
                    2=> Tccr2bBuilder(0).cs22().set_bit().build(),
                    _=> 0,
                }
            },
            Prescaling::_1024=>{
                match N {
                    0=> Tccr0bBuilder(0).cs00().set_bit().cs02().set_bit().build(),
                    1=> Tccr1bBuilder(0).cs10().set_bit().cs12().set_bit().build(),
                    2=> Tccr2bBuilder(0).cs20().set_bit().cs22().set_bit().build(),
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
                    0=>Tccr0aBuilder(0).build(),
                    1=>Tccr1aBuilder(0).build(),
                    2=>Tccr2aBuilder(0).build(),
                    _=>0,
                }
            },
            Mode::CTC=>{
                match N{
                    0=>Tccr0aBuilder(0).wgm01().set_bit().build(),
                    1=>Tccr1aBuilder(0).wgm11().set_bit().build(),
                    2=>Tccr2aBuilder(0).wgm21().set_bit().build(),
                    _=>0,
                }
            },
            Mode::PwdFast=>{
                match N{
                    0=>Tccr0aBuilder(0).wgm00().set_bit().wgm01().set_bit().build(),
                    1=>Tccr1aBuilder(0).wgm10().set_bit().wgm11().set_bit().build(),
                    2=>Tccr2aBuilder(0).wgm20().set_bit().wgm21().set_bit().build(),
                    _=>0,
                }
            },
            Mode::PwdPhaseCorrect=>{
                match N{
                    0=>Tccr0aBuilder(0).wgm00().set_bit().build(),
                    1=>Tccr1aBuilder(0).wgm10().set_bit().build(),
                    2=>Tccr2aBuilder(0).wgm20().set_bit().build(),
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
