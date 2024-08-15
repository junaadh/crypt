extern crate proc_macro;

use cryptic_codable::impl_codable;
use cryptic_error::impl_cryptee;
use proc_macro::TokenStream;

mod cryptic_codable;
mod cryptic_error;

#[proc_macro_derive(Cryptee)]
pub fn derive_error(item: TokenStream) -> TokenStream {
    impl_cryptee(item)
}

#[proc_macro_derive(Codable, attributes(code))]
pub fn derive_codable(item: TokenStream) -> TokenStream {
    impl_codable(item)
}
