pub mod master;
pub mod twi;
pub enum StatusCode {}
impl StatusCode {
    pub const STARTED: u8 = 0x08;
    pub const START_REPEATED: u8 = 0x10;

    pub const DATA_W_ACK: u8 = 0x50;
    pub const DATA_W_NACK: u8 = 0x58;

    pub const DATA_R_ACK: u8 = 0x28;
    pub const DATA_R_NACK: u8 = 0x30;

    pub const SLA_W_ACK: u8 = 0x18;
    pub const SLA_W_NACK: u8 = 0x20;

    pub const SLA_R_ACK: u8 = 0x40;
    pub const SLA_R_NACK: u8 = 0x48;

    pub const LOST: u8 = 0x38;
}
