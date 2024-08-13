use std::fmt::Display;

use cryptic_derive::Cryptee;

#[derive(Cryptee)]
pub enum VmError {
    /// vm interrupted
    Interrupt,
    ///invalid register: 0x{:02x}
    InvalidRegisterNo(u8),
    ///invalid register: {}
    InvalidRegisterStr(Box<dyn Display>),
}
