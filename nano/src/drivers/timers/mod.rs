use crate::utils::register::*;

pub trait RegisterSelection<const N:u8>{ type RegType; }
impl RegisterSelection<0> for (){ type RegType = Register; }
impl RegisterSelection<1> for (){ type RegType = RegisterU16; }
impl RegisterSelection<2> for (){ type RegType = Register; }

pub mod timer;
pub mod action;
pub mod interrupt;
pub mod callback;
pub mod settings;
