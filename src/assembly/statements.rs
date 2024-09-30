use crate::assembly::DEFAULT_WHITESPACE;

use super::Symbol;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statements<'a> {
    DPI {
        instruction: Symbol<'a>,
        op1: Symbol<'a>,
        op2: Symbol<'a>,
        op3: Option<Symbol<'a>>,
    },
    LSI {
        instruction: Symbol<'a>,
        op1: Symbol<'a>,
        obracket: bool,
        op2: Symbol<'a>,
        cbracket: bool,
        op3: Symbol<'a>,
    },
    BRI {
        instruction: Symbol<'a>,
        label: Symbol<'a>,
    },
    SCI {
        instruction: Symbol<'a>,
        vector: Symbol<'a>,
    },
    Directive {
        name: Symbol<'a>,
        params: Vec<Symbol<'a>>,
        body: Vec<Statements<'a>>,
        marker: Option<Symbol<'a>>,
        pc: u32,
    },
    Substitution {
        name: Symbol<'a>,
        values: Vec<Symbol<'a>>,
    },
    Label {
        name: Symbol<'a>,
    },
    Comment {
        name: Symbol<'a>,
    },
    #[default]
    Eof,
}

impl<'a> Statements<'a> {
    pub fn resolve(&self, params: Vec<Symbol<'a>>, values: Vec<Symbol<'a>>) -> Self {
        assert_eq!(params.len(), values.len());

        fn resolve_field<'a>(
            sym: &Symbol<'a>,
            replacement: &[(Symbol<'a>, Symbol<'a>)],
        ) -> Symbol<'a> {
            for (param, value) in replacement {
                // println!("param: {param:#?}\nval: {value:#?}\nsym: {sym:#?}");
                if sym.lexeme() == param.lexeme() {
                    return value.clone();
                }
            }
            sym.clone()
        }

        let fields = params
            .iter()
            .zip(values.iter())
            .map(|x| (x.0.clone(), x.1.clone()))
            .collect::<Vec<(Symbol<'a>, Symbol<'a>)>>();

        match self {
            Self::DPI {
                instruction,
                op1,
                op2,
                op3,
            } => {
                let op_1 = resolve_field(op1, fields.as_slice());
                let op_2 = resolve_field(op2, fields.as_slice());
                let op_3 = op3
                    .as_ref()
                    .map(|op3_val| resolve_field(op3_val, fields.as_slice()));
                // println!("{op_1:#?}\n{op_2:#?}\n{op_3:#?}");
                Statements::DPI {
                    instruction: instruction.clone(),
                    op1: op_1,
                    op2: op_2,
                    op3: op_3,
                }
            }
            Self::LSI {
                instruction,
                obracket,
                op1,
                op2,
                cbracket,
                op3,
            } => {
                let op_1 = resolve_field(op1, fields.as_slice());
                let op_2 = resolve_field(op2, fields.as_slice());
                let op_3 = resolve_field(op3, fields.as_slice());
                Self::LSI {
                    instruction: instruction.clone(),
                    op1: op_1,
                    obracket: *obracket,
                    op2: op_2,
                    cbracket: *cbracket,
                    op3: op_3,
                }
            }
            Self::BRI { instruction, label } => {
                let op_1 = resolve_field(label, fields.as_slice());
                Self::BRI {
                    instruction: instruction.clone(),
                    label: op_1,
                }
            }
            Self::SCI {
                instruction,
                vector,
            } => {
                let vector = resolve_field(vector, fields.as_slice());
                Self::SCI {
                    instruction: instruction.clone(),
                    vector,
                }
            }
            _ => unreachable!("substitution macros are only able to expand to instructions"),
        }
    }
}

impl<'a> fmt::Display for Statements<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DPI {
                instruction,
                op1,
                op2,
                op3,
            } => {
                if op3.is_some() {
                    write!(
                        f,
                        "\t{:<6}{DEFAULT_WHITESPACE}{}, {}, {}",
                        instruction,
                        op1,
                        op2,
                        op3.as_ref().unwrap()
                    )
                } else {
                    write!(
                        f,
                        "\t{:<6}{DEFAULT_WHITESPACE}{}, {}",
                        instruction, op1, op2
                    )
                }
            }
            Self::LSI {
                instruction,
                op1,
                op2,
                cbracket,
                op3,
                ..
            } => {
                if *cbracket {
                    write!(
                        f,
                        "\t{:<6}{DEFAULT_WHITESPACE}{}, [{}], {}",
                        instruction, op1, op2, op3
                    )
                } else {
                    write!(
                        f,
                        "\t{:<6}{DEFAULT_WHITESPACE}{}, [{}, {}]",
                        instruction, op1, op2, op3
                    )
                }
            }
            Self::BRI { instruction, label } => write!(
                f,
                "\t{:>06}{DEFAULT_WHITESPACE}#0x{:02x}\t; {}",
                instruction,
                label.pc(),
                label.lexeme()
            ),
            Self::SCI {
                instruction,
                vector,
            } => writeln!(f, "\t{:<6}{DEFAULT_WHITESPACE}{}", instruction, vector),
            Self::Directive {
                name,
                params,
                body,
                marker,
                ..
            } => {
                let params = params
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                if marker.is_some() && !body.is_empty() {
                    let body = body
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("\n");
                    write!(
                        f,
                        ".{:<6}{}{}\n{}\n{}",
                        name,
                        DEFAULT_WHITESPACE,
                        params,
                        body,
                        marker.as_ref().unwrap()
                    )
                } else {
                    write!(f, ".{:<6}{}{}", name, DEFAULT_WHITESPACE, params)
                }
            }
            Self::Substitution { name, values } => {
                let values = values
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "\t{:<6}{}{}", name, DEFAULT_WHITESPACE, values)
            }
            Self::Label { name } => write!(f, "\n{}:", name),
            Self::Comment { name } => write!(f, "{}", name),
            Self::Eof => write! {f, "\\EOF"},
        }
    }
}
