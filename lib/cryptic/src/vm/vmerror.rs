use std::fmt::Display;

use cryptic_derive::Cryptee;

#[derive(Cryptee)]
pub enum VmError {
    /// vm interrupted
    Interrupt,
    ///invalid register: {}
    InvalidRegister(Box<dyn Display + 'static>),
    /// Unable to parse token: {}
    ParseErr(Box<dyn Display + 'static>),
}
