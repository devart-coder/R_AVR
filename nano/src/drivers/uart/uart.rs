use super::{input::Input,output::Output,interrupt::Interrupt,settings::Settings};

pub struct Uart{
    pub settings:Settings,
    pub interrupt:Interrupt,
    pub output:Output,
    pub input:Input,
}
impl Uart{
    pub const fn new()->Self{
        Uart {
            settings: Settings::new(),
            interrupt: Interrupt::new(),
            output: Output::new(),
            input:Input::new(),
        }
    }
}
pub const uart:Uart = Uart::new();
