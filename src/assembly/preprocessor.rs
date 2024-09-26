use std::{borrow::Cow, collections::HashMap};

use crate::{assembly::Scanner, error::EsiuxErrorKind, format::Section, Res};

use super::{Function, Macros, Statements, SubMacro, Token};

pub const DEFAULT_WHITESPACE: &str = "    ";

#[derive(Debug, Default, Clone)]
pub struct PreProcessor<'a> {
    pub labels: HashMap<Cow<'a, str>, u32>,
    pub variables: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub macros: HashMap<Cow<'a, str>, Macros<'a>>,
    pub pc: u32,
    pub source: &'a str,
    pub section: Section,
    pub entry: Option<Cow<'a, str>>,
    pub intern_buf: Vec<Statements<'a>>,
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

    fn get_macro(&mut self, name: &str) -> Option<Macros<'a>> {
        self.macros.get(name).cloned()
    }

    pub fn define_submacro(&mut self, name: Cow<'a, str>, value: SubMacro<'a>) {
        self.macros.insert(name, Macros::Substitution(value));
    }

    pub fn handle(&mut self) -> Res<()> {
        let mut st = Vec::new();
        let mut scan = Scanner::new(self.source);
        for stmt in scan.analyze() {
            // print!("{}\npc: {}", stmt, self.pc);
            match stmt.clone() {
                Statements::DPI { .. }
                | Statements::LSI { .. }
                | Statements::SCI { .. }
                | Statements::BRI { .. } => {
                    self.pc += 4;
                    st.push(stmt);
                }
                Statements::Comment { .. } => {
                    st.push(stmt);
                }
                Statements::Directive { name, .. } | Statements::Substitution { name, .. } => {
                    let mac = self.get_macro(name.lexeme().trim_start_matches("."));
                    let resolved = match mac {
                        Some(Macros::Directive(func)) => func(self, stmt),
                        Some(Macros::Substitution(sub)) => {
                            let SubMacro {
                                input,
                                body,
                                offset,
                            } = sub;
                            let val = if let Statements::Substitution { values, .. } = stmt.clone()
                            {
                                values
                            } else {
                                panic!("This shudnt happend: submacro");
                            };

                            let mut st_inner = Vec::new();
                            for stmt in body {
                                let res = stmt.resolve(input.clone(), val.clone());
                                st_inner.push(res);
                            }

                            self.pc += offset;
                            Ok(st_inner)
                        }
                        _ => {
                            return Err(EsiuxErrorKind::UnknownDirective(
                                name.lexeme().to_string(),
                                name.line(),
                            ))
                        }
                    }?;
                    st.extend_from_slice(&resolved);
                }
                Statements::Label { name } => {
                    self.labels.insert(name.lexeme(), name.pc());
                    st.push(stmt);
                }
                Statements::Eof => st.push(stmt),
            }
            // println!("new: {}", self.pc);
        }

        st.iter_mut().for_each(|x| {
            if let Statements::BRI { label, .. } = x {
                let new_pc = self.labels.get(&label.lexeme()).unwrap();
                let nlabel = super::Symbol::Label(Token {
                    lexeme: label.lexeme(),
                    offset: 0,
                    len: 0,
                    line: 0,
                    pc: Some(*new_pc),
                });
                *label = nlabel;
            }
        });

        self.intern_buf.extend_from_slice(&st);
        Ok(())
    }
}
