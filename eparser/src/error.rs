use std::fmt::Display;

use emacro::Error;

#[derive(Error)]
pub enum ParserErrorKind {
    /// Unknown token: {}
    UnknownToken(Box<dyn Display>),
    /// Unexpected symbol: {}, expected: {}
    Unexpected(Box<dyn Display>, Box<dyn Display>),
    /// Unexpected end of file
    Eof,
}
