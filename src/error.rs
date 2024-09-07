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
}

impl From<ParseIntError> for EsiuxErrorKind {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(Box::new(value))
    }
}
