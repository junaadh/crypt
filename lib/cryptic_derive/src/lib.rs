extern crate proc_macro;
use cryptic_error::impl_cryptee;
use proc_macro::TokenStream;

mod cryptic_error;

#[proc_macro_derive(Cryptee)]
pub fn derive_error(item: TokenStream) -> TokenStream {
    impl_cryptee(item)
}
