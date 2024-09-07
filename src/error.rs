use std::{fmt::Display, num::ParseIntError};

use emacro::Error;

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
}

impl From<ParseIntError> for EsiuxErrorKind {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(Box::new(value))
    }
}
