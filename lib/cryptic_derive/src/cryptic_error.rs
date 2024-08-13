use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Ident, Lit, Meta, MetaNameValue};

pub fn impl_cryptee(tt: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tt as DeriveInput);
    let name = &input.ident;

    let data = match input.data {
        Data::Enum(e) => e,
        _ => panic!("This macro derives only enums"),
    };

    let mut display = Vec::new();
    let mut debug = Vec::new();

    data.variants.iter().for_each(|x| {
        let variant = &x.ident;
        let msg = x
            .attrs
            .iter()
            .find_map(|at| match &at.meta {
                Meta::NameValue(MetaNameValue {
                    path,
                    eq_token: _,
                    value:
                        Expr::Lit(ExprLit {
                            attrs: _,
                            lit: Lit::Str(x),
                        }),
                }) if path.is_ident("doc") => Some(x.value()),
                _ => None,
            })
            .expect("Each field requires a doc comment error message");

        let fields = x
            .fields
            .iter()
            .enumerate()
            .map(|(index, _)| {
                Ident::new(
                    format!("a{}", index).as_str(),
                    proc_macro2::Span::call_site(),
                )
            })
            .collect::<Vec<_>>();

        if fields.is_empty() {
            display.push(quote! {
                Self::#variant => write!(f, #msg),
            });

            debug.push(quote! {
                Self::#variant => write!(f, #msg),
            })
        } else if fields.len() == 1 {
            let msg_populated = msg.replace("{", "{a0");
            display.push(quote! {
                Self::#variant(#(#fields),*) => write!(f, #msg_populated),
            });

            debug.push(quote! {
                Self::#variant(#(#fields),*) => write!(f, #msg_populated),
            })
        } else {
            display.push(quote! {
                Self::#variant(#(#fields),*) => write!(f, #msg, #(#fields),*),
            });

            debug.push(quote! {
                Self::#variant(#(#fields),*) => write!(f, #msg, #(#fields),*),
            })
        }
    });
    quote! {
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
                    #(#debug)*
                }
            }
        }
    }
    .into()
}
