pub mod asm_token;
pub mod error;
pub mod lexer;
pub mod parser;

pub type ParseRes<T> = Result<T, error::ParserErrorKind>;
