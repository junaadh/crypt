use cryptic_derive::Cryptee;
use std::fmt::Display;

#[derive(Cryptee)]
pub enum AsmError {
    /// failed to read file: {}
    ReadError(Box<dyn Display>),
    /// failed to parse value: {}
    ParseError(Box<dyn Display>),
}
