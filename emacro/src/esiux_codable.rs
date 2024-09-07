use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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

    let mut init_ = Vec::<proc_macro2::TokenStream>::new();
    let mut from_str = Vec::<proc_macro2::TokenStream>::new();
    let mut from_u8 = Vec::<proc_macro2::TokenStream>::new();
    let mut display = Vec::<proc_macro2::TokenStream>::new();
    let mut match_ = Vec::<proc_macro2::TokenStream>::new();
    let mut decode_ = Vec::<proc_macro2::TokenStream>::new();

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

        let types = variant
            .fields
            .iter()
            .map(|x| x.ty.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .expect("Expected one value");

        // println!("{types}");

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

        init_.push(quote! {
            #variant_name = #number,
        });

        match_.push(quote! {
            #variant_name => #variant_name(it),
        });

        let decode_map = match types.as_str() {
            "DPI" => quote! {
                Op::#variant_name => Ok(#name::#variant_name(crate::processor::DPI::try_from(value)?)),
            },
            "LSI" => quote! {
                Op::#variant_name => Ok(#name::#variant_name(crate::processor::LSI::try_from(value)?)),
            },
            "BRI" => quote! {
                Op::#variant_name => Ok(#name::#variant_name(crate::processor::BRI::try_from(value)?)),
            },
            "SCI" => quote! {
                Op::#variant_name => Ok(#name::#variant_name(crate::processor::SCI::try_from(value)?)),
            },
            _ => panic!("Unexpected type fuck.."),
        };
        decode_.push(decode_map);
    }

    quote! {
        use #path;

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum Op {
            #(#init_)*
        }

        impl std::str::FromStr for Op {
            type Err = #error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.to_lowercase();
                match s.as_str() {
                    #(#from_str)*
                    _ => Err(Self::Err::FromStr(Box::new(format!("Failed to parse {s} to Op")))),
                }
            }
        }

        impl std::convert::TryFrom<u32> for #name {
            type Error = #error;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                let ins = ((value >> 4) & 0b111) as u8;
                let ins = match ins {
                    0x1 | 0x5 | 0x7 => ((value >> 8) & 0xf) as u8 | ins << 4,
                    0x3 => ((value >> 11) & 0b1) as u8 | ins << 4,
                    _ => panic!("This shouldnt happen"),
                };
                let ins = Op::try_from(ins)?;

                match ins {
                    #(#decode_)*
                    _ => Err(Self::Error::Decode(value)),
                }
            }
        }

        impl std::convert::TryFrom<u8> for Op {
            type Error = #error;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#from_u8)*
                    _ => Err(Self::Error::TryFrom(Box::new(format!("Failed to parse {value} to Op")))),
                }
            }
        }

        impl std::fmt::Display for Op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display)*
                }
            }
        }

        impl std::fmt::Debug for Op {
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
