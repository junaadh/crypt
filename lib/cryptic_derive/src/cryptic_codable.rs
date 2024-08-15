use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parenthesized, parse::Parse, parse_macro_input, Data, DeriveInput, Ident, Lit, LitInt, LitStr,
    Meta, Token,
};

pub fn impl_codable(tt: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tt as DeriveInput);

    let name = &input.ident;
    let data = match input.data {
        Data::Enum(e) => e,
        _ => panic!("Requries an enum to derive Codable"),
    };

    let mut code_init = Vec::new();
    let mut code_fnum = Vec::new();
    let mut code_fstr = Vec::new();

    for variant in data.variants {
        let variant_name = &variant.ident;

        let types = if variant.fields.is_empty() {
            Vec::new()
        } else {
            variant
                .fields
                .iter()
                .map(|field| field.ty.to_token_stream().to_string())
                .collect::<Vec<_>>()
        };

        let mut tokens = None::<Tokens>;

        for attr in &variant.attrs {
            if attr.path().is_ident("code") {
                tokens = Some(
                    attr.parse_args::<Tokens>()
                        .expect("Failed to parse token attribute"),
                );
            }
        }

        let Tokens { number, mnumonic } = tokens.unwrap();

        code_init.push(quote! {
            #variant_name = #number,
        });

        code_fnum.push(quote! {
            #number => Ok(Code::#variant_name),
        });

        code_fstr.push(quote! {
            #mnumonic => Ok(Code::#variant_name),
        });

        if types.is_empty() {
            todo!()
        }
        match types.iter().map(|x| x.as_str()).collect::<Vec<_>>()[..] {
            ["u8"] => todo!(),
            _ => panic!("Dunno how to handle these types"),
        }
    }

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum Code {
            #(#code_init)*
        }

        impl std::convert::TryFrom<u8> for Code {
            type Error = crate::asm::AsmError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#code_fnum)*
                    _ => Err(Self::Error::ParseError(Box::new(value))),
                }
            }
        }

        impl std::str::FromStr for Code {
            type Err = crate::asm::AsmError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#code_fstr)*
                    _ => Err(Self::Err::ParseError(Box::new(s.to_string()))),
                }
            }
        }

    }
    .into()
}

#[derive(Debug)]
struct Tokens {
    number: u8,
    mnumonic: String,
}

impl Parse for Tokens {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let opcode = input.parse::<LitInt>()?;
        input.parse::<Token![,]>()?;
        let mnumonic = input.parse::<LitStr>()?;
        Ok(Tokens {
            number: opcode.base10_parse::<u8>()?,
            mnumonic: mnumonic.value(),
        })
    }
}
