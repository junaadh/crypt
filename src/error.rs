use std::{fmt::Display, io, num::ParseIntError};

use emacro::Error;
use eparser::error::ParserErrorKind;

/// # EsiuxErrorKind
///
/// * Contains all the error types
/// * implements error types from common error types
/// * implements emacro::Error which accepts arbitrary arguments
/// * arguments need to implement Display and have a static lifetime
///
#[derive(Error)]
pub enum EsiuxErrorKind {
    /// Unable to parse token : {}
    ParseInt(Box<dyn Display + 'static>),
    /// Unable to match number: {}
    TryFrom(Box<dyn Display + 'static>),
    /// Unable to parse from string: {}
    FromStr(Box<dyn Display + 'static>),
    /// Unable to decode instruction: {} ; {:032b}
    Decode(u32),
    /// Maximum value l12 can hold is 4095 provided is {}
    Overflow12(u16),
    /// Maximum value l20 can hold is 1048575 provided is {}
    Overflow20(u32),
    /// {} requires atleast {} parts
    NotEnoughParts(Box<dyn Display + 'static>, u8),
    /// Tried to access mem: {:02x} which is out of range: {:032b}
    MemOutOfBounds(u32),
    /// Failed to access i/o: {}
    Io(io::Error),
    /// Binary file does not contain any raw data
    EmptyBin,
    /// Invalod {}, expected: {}, found {}
    Invalid(String, usize, usize),
    /// No arguments provided for macro definition
    DefineMacro,
    /// Unrecognized token: {}
    UnknownToken(Box<dyn Display>),
    /// Invalid macro format - {}
    Format(Box<dyn Display>),
    /// {}
    Lexer(ParserErrorKind),
    /// Unknown symbol: {} at line {}:{}
    UnknownSymbol(char, usize, usize),
}

impl From<ParseIntError> for EsiuxErrorKind {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(Box::new(value))
    }
}

impl From<io::Error> for EsiuxErrorKind {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParserErrorKind> for EsiuxErrorKind {
    fn from(value: ParserErrorKind) -> Self {
        Self::Lexer(value)
    }
}
