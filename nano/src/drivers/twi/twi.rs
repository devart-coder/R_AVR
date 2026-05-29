use crate::{ mcu::registers::{twcr::{Twcr, TwcrBuilder}, twdr::Twdr}, utils::register::Register};

pub struct Twi{
    twcr:Register,
    twdr:Register,
}
//UserInterface
//twi
//.master()
//.start()
//.write(u8)
//.read() || .read_ack()
//.stop();

//Private
//read_status(u8);
//read_int_flag();
impl Twi{
    const READ:u8 = 1;
    const WRITE:u8 = 0;
    pub fn new()->Self{
        Self{
            twcr : Register::new(Twcr::address()),
            twdr : Register::new(Twdr::address()),
        }
    }
    fn _check_twint_flag_set(self){
        let value = 
        TwcrBuilder(0)
        .twint().set_bit().build();
        while(self.twcr.read() & value) == 0 {};
    }
    // fn _check_status_code(self,status:StatusCode)->bool{
    //     if (self.twcr.read() & 0xF8) != status {
    //         false
    //     } else{
    //         true
    //     }
    // }
    
    pub fn start(self)->Self{
        let value = 
            TwcrBuilder(0)
            .twen().set_bit()
            .twsta().set_bit()
            .twint().set_bit()
            .build();
        self.twcr.modify(|v| v|value);
        self
    }
    pub fn stop(self)->Self{
        let value = 
            TwcrBuilder(0)
            .twen().set_bit()
            .twsto().set_bit()
            .twint().set_bit()
            .build();
        self.twcr.modify(|v| v|value);
        self
    }
    pub fn write_sla_w(self,val:u8)->Self{
        self.write_data(val << 1 | Self::WRITE)
    }
    pub fn write_sla_r(self,val:u8)->Self{
        self.write_data(val << 1 | Self::READ)
    }
    pub fn write_data(self,val:u8)->Self{
        self.twdr.write(val);
        let value = 
            TwcrBuilder(0)
            .twint().set_bit()
            .twen().set_bit()
            .build();
        self.twcr.modify(|v| v|value);
        self
    }
}
//StatusCode(Master::Rx::Started)
//StatusCode(Master::Tx::Started)
//StatusCode(Slave::Tx::Started)
//StatusCode(Slave::Rx::Started)
enum StatusCode{
    Master(Master),
}
enum Master{
    Rx(MasterRx),
    Tx(MasterTx),
}

enum MasterTx{
    Started         = 0x08,
    StartRepeated   = 0x10,
    SlawAck         = 0x18,
    SlawNack        = 0x20,
    DataAck         = 0x28,
    DataNack        = 0x30,
    Lost            = 0x38,
}

enum MasterRx{
    Started         = 0x08,
    StartRepeated   = 0x10,
    SlarLost        = 0x38,
    DataAck         = 0x50,
    DataNack        = 0x58,
}