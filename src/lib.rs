#[macro_use]
mod macros;
pub mod assembly;
pub mod error;
pub mod format;
pub mod machine;
pub mod memory;
pub mod parser;
pub mod processor;
pub mod types;

pub type Res<T> = Result<T, error::EsiuxErrorKind>;
