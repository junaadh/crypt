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
    let mut parse_ = Vec::<proc_macro2::TokenStream>::new();
    let mut debug_instruction = Vec::<proc_macro2::TokenStream>::new();
    let mut encode_ = Vec::<proc_macro2::TokenStream>::new();

    let mut all_op = String::new();
    all_op.push('_');

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

        all_op.push_str(mnumonic);
        all_op.push('_');

        from_str.push(quote! {
            x if x.starts_with(#mnumonic) => Ok(Self::#variant_name),
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

        debug_instruction.push(quote! {
            Self::#variant_name(i) => write!(f, "{}", i),
        });


        encode_.push(quote! {
            Self::#variant_name(i) => i.mask(),
        });

        match types.as_str() {
            "DPI" => {
                decode_.push(quote! {
                    Op::#variant_name => Ok(#name::#variant_name(crate::processor::DPI::try_from(value)?)),
                });
                parse_.push(quote! {
                    Op::#variant_name => {
                        if !(Op::#variant_name == Op::Mov || Op::#variant_name == Op::Cmp) {
                            if parts.len() < 3 { 
                                return Err(crate::error::EsiuxErrorKind::NotEnoughParts(
                                    Box::new(instruction_parsed),
                                    3,
                                ));
                            }
                        } else if parts.len() < 2 {
                            return Err(crate::error::EsiuxErrorKind::NotEnoughParts(
                                Box::new(instruction_parsed),
                                2,
                            ));
                        }

                        let rd = parts[0].parse::<crate::processor::Register>()?;
                        let (op, rn) = if !(Op::#variant_name == Op::Mov || Op::#variant_name == Op::Cmp) {   
                            let rn = parts[1].parse::<crate::processor::Register>()?;
                            let op = parts[2].parse::<crate::types::Operand>()?;
                            (op, rn)
                        } else {
                            let rn = crate::processor::Register::R0;
                            let op = parts[1].parse::<crate::types::Operand>()?;
                            (op, rn)
                        };

                        let dpi = instruction.mk_instruction::<crate::processor::DPI>(instruction_parsed, rd, rn, op)?;

                        Ok(Self::#variant_name(dpi))
                    },
                })
            }
            "LSI" => {
                decode_.push(quote! {
                    Op::#variant_name => Ok(#name::#variant_name(crate::processor::LSI::try_from(value)?)),
                });
                parse_.push(quote! {
                    Op::#variant_name => {                    
                        if parts.len() < 2 {
                            return Err(crate::error::EsiuxErrorKind::NotEnoughParts(
                                Box::new(instruction_parsed),
                                2,
                            ));
                        }

                        let mut index = false;
                        let mut val = crate::types::l12::new_u(0)?;

                        let rd = parts[0];
                        let mut rn = &parts[1][1..];

                        if let Some(num) = parts.get(2) {
                            if num.ends_with("]") {
                                val = num[1..num.len() - 1].parse::<crate::types::l12>()?;
                            } else {
                                index = true;
                                rn = &rn[..rn.len() - 1];
                                val = num[1..].parse::<crate::types::l12>()?;
                            }
                        } else {
                            rn = &rn[..rn.len() - 1];
                        }

                        let rd = rd.parse::<crate::processor::Register>()?;
                        let rn = rn.parse::<crate::processor::Register>()?;

                        let mut lsi = instruction.mk_instruction::<crate::processor::LSI>(instruction_parsed, rd, rn, val)?;

                        lsi.index = index;

                        Ok(Self::#variant_name(lsi))
                    },       
                })
            }
            "BRI" => {
                decode_.push(quote! {
                    Op::#variant_name => Ok(#name::#variant_name(crate::processor::BRI::try_from(value)?)),
                });
                parse_.push(quote! {
                    Op::#variant_name => {
                        if parts.len() < 1 {
                            return Err(crate::error::EsiuxErrorKind::NotEnoughParts(
                                Box::new(instruction_parsed),
                                1,
                            ));
                        }

                        let offset = parts[0][1..].parse::<crate::types::l20>()?;

                        let bri = instruction.mk_instruction::<crate::processor::BRI>(
                            instruction_parsed,
                            crate::processor::Register::R0,
                            crate::processor::Register::R0,
                            offset,
                        )?;

                        Ok(Self::#variant_name(bri))
                    },
                })
            }
            "SCI" => {
                decode_.push(quote! {
                    Op::#variant_name => Ok(#name::#variant_name(crate::processor::SCI::try_from(value)?)),
                });
                parse_.push(quote! {
                    Op::#variant_name => {
                        let int_key = parts[0][1..].parse::<crate::types::l12>()?.value as u8;

                        let sci = instruction.mk_instruction::<crate::processor::SCI>(
                            instruction_parsed,
                            crate::processor::Register::R0,
                            crate::processor::Register::R0,
                            int_key,
                        )?;

                        Ok(Self::#variant_name(sci))
                    },
                })
            }
            _ => panic!("Unexpected type fuck.."),
        };
    }

    quote! {
        use #path;
        use crate::parser::ParserImpl;

        pub fn get_all_op() -> String {
            #all_op.to_owned()
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum Op {
            #(#init_)*
        }

        impl std::str::FromStr for Op {
            type Err = #error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.to_lowercase();
                
                match s {
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
                    _ => panic!("This shouldnt happen: instruction_val: {ins} - {ins:08b}"),
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

        use crate::parser::ToNum;

        impl ToNum for #name {
            fn mask(&self) -> u32 {
                match self {
                    #(#encode_)*
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = #error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (instruction, parts) =
                    s.split_once(' ')
                        .ok_or(crate::error::EsiuxErrorKind::FromStr(Box::new(format!(
                            "requires a spaced delimeter: {s}"
                        ))))?;
                let parts = parts.split(',').map(|x| x.trim()).collect::<Vec<&str>>();

                let instruction_parsed = instruction.parse::<Op>()?;

                match instruction_parsed {
                    #(#parse_)*
                    _ => Err(crate::error::EsiuxErrorKind::FromStr(Box::new(format!(
                            "unrecognized instruction: {s}"
                        )))),
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

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#debug_instruction)*
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
