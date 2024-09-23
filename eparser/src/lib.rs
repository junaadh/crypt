pub mod error;
pub mod lexer;

pub type ParseRes<T> = Result<T, error::ParserErrorKind>;
