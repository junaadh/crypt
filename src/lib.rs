#[macro_use]
mod macros;
pub mod error;
pub mod parser;
pub mod processor;
pub mod types;

pub type Res<T> = Result<T, error::EsiuxErrorKind>;
