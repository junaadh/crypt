use std::{borrow::Cow, collections::HashMap};

use crate::{error::EsiuxErrorKind, format::Section, Res};

use super::{Function, Macros, Scanner, SubMacro, Symbol, SymbolStream, Token};

#[derive(Debug, Default, Clone)]
pub struct PreProcessor<'a> {
    pub labels: HashMap<Cow<'a, str>, u32>,
    pub variables: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub macros: HashMap<Cow<'a, str>, Macros<'a>>,
    pub pc: u32,
    pub source: &'a str,
    pub section: Section,
    pub entry: Option<Cow<'a, str>>,
}

impl<'a> PreProcessor<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut pp = Self {
            source,
            ..Default::default()
        };
        pp.standard_directives();
        pp
    }

    pub fn define_directive(&mut self, name: Cow<'a, str>, value: Function<'a>) {
        self.macros.insert(name, Macros::Directive(value));
    }

    pub fn define_std(&mut self, name: &'a str, value: Function<'a>) {
        let name = Cow::Borrowed(name);
        self.macros.insert(name, Macros::Directive(value));
    }

    pub fn define_submacro(&mut self, name: Cow<'a, str>, value: SubMacro<'a>) {
        self.macros.insert(name, Macros::Substitution(value));
    }

    pub fn handle(&'a mut self) -> Res<SymbolStream<'_>> {
        let mut st = SymbolStream::default();
        let mut cur_line = 0;
        for stream in Scanner::new(self.source).tokenize() {
            // println!("{stream:#?}");
            let mut instruction = false;
            let mut _pos = 0;
            for symbol in stream.0 {
                let sline = symbol.line();
                match sline {
                    x if x > cur_line => {
                        st.push(Symbol::Punct(Token::from("\n")));
                        cur_line = x;
                        _pos = 0;
                        instruction = false;
                    }
                    _ => {
                        _pos += 1;
                    }
                }
                // println!("{cur_line}:{pos}");
                match symbol {
                    Symbol::Label(Token {
                        lexeme,
                        pc: Some(val),
                        line,
                        ..
                    }) => {
                        self.labels.insert(lexeme.clone(), val as u32);
                        if instruction {
                            let lit = format!("#{val}");
                            st.push(Symbol::Literal(Token::from(lit)));
                            st.push(Symbol::Whitespace(Token::from("\t")));
                            st.push(Symbol::Comment(Token::from(lexeme)));
                        } else {
                            st.push(Symbol::Label(Token {
                                lexeme,
                                line,
                                pc: Some(val),
                                ..Default::default()
                            }));
                        }
                    }
                    Symbol::Directive(Token { lexeme, line, .. }, tokens) => {
                        // This should be empty coz directives dont push any tokens
                        // or do we push the tokens in the same order so as to recreate the og file???
                        let resolved = match self.macros.get(lexeme.as_ref()) {
                            Some(Macros::Directive(func)) => func(self, tokens)?,
                            Some(Macros::Substitution(_)) => {
                                return Err(EsiuxErrorKind::InvalidMacroMatch(lexeme.to_string()))
                            }
                            None => {
                                return Err(EsiuxErrorKind::UnknownDirective(
                                    lexeme.to_string(),
                                    line,
                                ))
                            }
                        };
                        st.extend_from_vec(resolved);
                    }
                    Symbol::Macros(Token { lexeme, line, .. }, tokens) => {
                        let resolved = match self.macros.get(lexeme.as_ref()) {
                            Some(Macros::Directive(_)) => {
                                return Err(EsiuxErrorKind::InvalidMacroMatch(lexeme.to_string()))
                            }
                            Some(Macros::Substitution(s)) => Vec::<Symbol<'a>>::new(),
                            None => {
                                return Err(EsiuxErrorKind::UnknownSubstitution(
                                    lexeme.to_string(),
                                    line,
                                ))
                            }
                        };
                        st.extend_from_vec(resolved);
                    }
                    Symbol::Instruction(t) => {
                        instruction = true;
                        self.pc = t.pc.unwrap_or_default() as u32;
                        st.push(Symbol::Instruction(t));
                    }
                    Symbol::Literal(s) => st.push(Symbol::Literal(s)),
                    Symbol::Register(s) => st.push(Symbol::Register(s)),
                    Symbol::Punct(s) => {
                        st.push(Symbol::Punct(s));
                    }
                    Symbol::Comment(s) => st.push(Symbol::Comment(s)),
                    // _ => todo!("wip: {symbol:?}"),
                    _ => {
                        println!("skipped: {symbol:#?}")
                    }
                }
            }
        }

        println!("{self:#?}");

        Ok(st)
    }
}
