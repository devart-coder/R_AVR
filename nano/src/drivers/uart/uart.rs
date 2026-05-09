use super::{input::Input,output::Output,interrupt::Interrupt,settings::Settings};

pub struct Uart{
    pub settings:Settings,
    pub interrupt:Interrupt,
    pub output:Output,
    pub input:Input,
}
impl Uart{
    const fn new()->Self{
        Uart {
            settings: Settings::new(),
            interrupt: Interrupt::new(),
            output: Output::new(),
            input:Input::new(),
        }
    }
    pub fn settings(&mut self)->&mut Settings{
        &mut self.settings
    }
    pub fn interrupt(&mut self)->&mut Interrupt{
        &mut self.interrupt
    }
    pub fn output(&mut self)->&mut Output{
        &mut self.output
    }
    pub fn input(&mut self)->&mut Input{
        &mut self.input
    }
    pub fn get()->&'static mut Uart{
        static mut U:Uart = Uart::new();
        unsafe { &mut *core::ptr::addr_of_mut!(U)}
    }
}
