extern crate proc_macro;

use esiux_codable::impl_codable;
use esiux_err::impl_cryptee;
use esiux_reg::impl_enum_from;
use proc_macro::TokenStream;

mod esiux_codable;
mod esiux_err;
mod esiux_reg;

/// # Error
///
/// * provides an easy way to represent errors
/// * leverages rust enum tuples
/// * Each enum variant is an error type
/// * variant associated doc comment acts as the message for the variant
/// * if variant is tuple need corresponfing number of fmt brackets "{}"
/// * if only one value in tuple multiple brackets can be used to print the value numerous times
/// * if not the values are positionally dependent
///
/// ``` rust
/// #[derive(Error)]
/// pub enum EmacroError {
///   /// This doesnt have any arbitrary values
///   Parse,
///   /// This contains one string: {}
///   Tokenization(Box<String>),
///   /// This is first print: {}, this is second print: {:02x}
///   State(u32),
///   /// This is two value: {} {}
///   Two(u32, Box<String>),
///   /// Can be anything here: {}
///   Anything(Box<dyn std::fmt::Display + 'static>)
/// }
///    
/// ```
///
#[proc_macro_derive(Error)]
pub fn derive_err(item: TokenStream) -> TokenStream {
    impl_cryptee(item)
}

/// # EnumFrom
///
/// * provides tryFrom for u8, u16, u32
/// * provides fromStr
/// * provides debug and display
/// * idx is the enumeration of the enum variants
/// * error type is provided by a meta error decorator
/// * code meta decorator for each variant is a &str divided by ','
///
/// ```rust
/// #[derive(EnumFrom)]
/// #[error(crate::error::EsiuxErrorKind)]
/// pub enum Register {
///   #[code("r0", "R0", "rzr", "RZR")]
///   R0
/// }
/// ```
///
#[proc_macro_derive(EnumFrom, attributes(code, error))]
pub fn derive_enum(item: TokenStream) -> TokenStream {
    impl_enum_from(item)
}

#[proc_macro_derive(Codable, attributes(alias, error))]
pub fn derive_codable(item: TokenStream) -> TokenStream {
    impl_codable(item)
}
