use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Data, DeriveInput, LitInt, LitStr, Token};

use crate::esiux_reg::ErrorType;

pub fn impl_codable(tt: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tt as DeriveInput);

    let data = match ast.data {
        Data::Enum(s) => s,
        _ => panic!("Requires an enum to derive Codable"),
    };

    let name = &ast.ident;
    let mut error = None::<ErrorType>;

    for attr in &ast.attrs {
        if attr.path().is_ident("error") {
            error = Some(attr.parse_args().expect("Failed to parse error"));
        }
    }

    let ErrorType { path, error } = error.unwrap();

    let mut from_str = Vec::<proc_macro2::TokenStream>::new();
    let mut from_u8 = Vec::<proc_macro2::TokenStream>::new();
    let mut display = Vec::<proc_macro2::TokenStream>::new();
    let mut as_u8 = Vec::<proc_macro2::TokenStream>::new();

    for variant in data.variants {
        let variant_name = &variant.ident;

        let mut alias = None::<Alias>;

        for attr in variant.attrs {
            if attr.path().is_ident("alias") {
                alias = Some(
                    attr.parse_args::<Alias>()
                        .expect("Failed to parse alias attribute"),
                );
            }
        }

        let Alias { mnumonic, number } = alias.unwrap();
        let mnumonic = mnumonic.as_str();

        from_str.push(quote! {
            #mnumonic => Ok(Self::#variant_name),
        });

        from_u8.push(quote! {
            #number => Ok(Self::#variant_name),
        });

        display.push(quote! {
            Self::#variant_name => write!(f, "{}", #mnumonic),
        });

        as_u8.push(quote! {
            Self::#variant_name => #number,
        });
    }

    quote! {
        use #path;

        impl #name {
            pub fn as_u8(&self) -> u8 {
                match self {
                    #(#as_u8)*
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = #error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.to_lowercase();
                match s.as_str() {
                    #(#from_str)*
                    _ => Err(Self::Err::FromStr(Box::new(s.to_string()))),
                }
            }
        }

        impl std::convert::TryFrom<u8> for #name {
            type Error = #error;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#from_u8)*
                    _ => Err(Self::Error::TryFrom(Box::new(value))),
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display)*
                }
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display)*
                }
            }
        }
    }
    .into()
}

struct Alias {
    mnumonic: String,
    number: u8,
}

impl Parse for Alias {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mnumonic = input.parse::<LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let number = input.parse::<LitInt>()?.base10_parse::<u8>()?;
        Ok(Self { mnumonic, number })
    }
}
