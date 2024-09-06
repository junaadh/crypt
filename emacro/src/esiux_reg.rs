use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, Data, DeriveInput, Ident, LitStr, Meta, MetaList, Path,
};

pub fn impl_enum_from(tt: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tt as DeriveInput);

    let ident = &ast.ident;
    let mut error = None::<ErrorType>;

    for attr in ast.attrs {
        if attr.path().is_ident("error") {
            let a = attr.parse_args().expect("Failed to parse args");
            error = Some(a);
        }
    }

    let ErrorType { path, error } = error.unwrap();

    let data = match ast.data {
        Data::Enum(e) => e,
        _ => panic!("Requires an enum to derive Error"),
    };

    let mut from_num_u8 = Vec::<proc_macro2::TokenStream>::new();
    let mut from_num_u16 = Vec::<proc_macro2::TokenStream>::new();
    let mut from_num_u32 = Vec::<proc_macro2::TokenStream>::new();
    let mut from_str = Vec::<proc_macro2::TokenStream>::new();
    let mut as_str = Vec::<proc_macro2::TokenStream>::new();

    for (idx, variant) in data.variants.iter().enumerate() {
        let variant_name = &variant.ident;

        let mut tokens_vec = proc_macro2::TokenStream::new();
        let mut str = Vec::new();

        for attr in variant.attrs.iter() {
            if attr.path().is_ident("code") {
                if let Meta::List(MetaList {
                    path: _,
                    delimiter: _,
                    tokens,
                }) = &attr.meta
                {
                    tokens_vec = tokens.to_token_stream();
                } else {
                    panic!("Should be a list")
                }
            }
        }

        for token in tokens_vec {
            if let TokenTree::Literal(lit) = token {
                let literal =
                    syn::parse_str::<LitStr>(&lit.to_string()).expect("Failed to parse string");
                str.push(literal.value());
            }
        }

        let str = str.iter().map(|x| x.as_str()).collect::<Vec<_>>();
        let string = str[0];

        let idx_u8 = idx as u8;
        from_num_u8.push(quote! {
            #idx_u8 => Ok(#ident::#variant_name),
        });

        let idx_u16 = idx as u16;
        from_num_u16.push(quote! {
            #idx_u16 => Ok(#ident::#variant_name),
        });

        let idx_u32 = idx as u32;
        from_num_u32.push(quote! {
            #idx_u32 => Ok(#ident::#variant_name),
        });

        from_str.push(quote! {
            #(#str)|* => Ok(#ident::#variant_name),
        });

        as_str.push(quote! {
            #ident::#variant_name => #string,
        });
    }

    quote! {
        use #path;

        impl #ident {
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str)*
                }
            }
        }

        impl std::convert::TryFrom<u8> for #ident {
            type Error = #error;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#from_num_u8)*
                    _ => Err(#error::TryFrom(Box::new(value))),
                }
            }
        }

        impl std::convert::TryFrom<u16> for #ident {
            type Error = #error;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    #(#from_num_u16)*
                    _ => Err(#error::TryFrom(Box::new(value))),
                }
            }
        }

        impl std::convert::TryFrom<u32> for #ident {
            type Error = #error;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    #(#from_num_u32)*
                    _ => Err(#error::TryFrom(Box::new(value))),
                }
            }
        }

        impl std::str::FromStr for #ident {
            type Err = #error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#from_str)*
                    _ => Err(#error::FromStr(Box::new(s.to_string()))),
                }
            }
        }

        impl std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    }
    .into()
}

#[derive(Debug)]
pub struct ErrorType {
    pub path: Path,
    pub error: Ident,
}

impl Parse for ErrorType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>()?;
        let error = path.segments.last().unwrap().ident.clone();
        Ok(Self { path, error })
    }
}
