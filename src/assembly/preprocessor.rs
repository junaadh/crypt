use std::{borrow::Cow, collections::HashMap};

use crate::{format::Section, Res};

use super::{Function, Macros, Scanner, SubMacro, Symbol, Token};

#[derive(Debug, Default)]
pub struct PreProcessor<'a> {
    pub labels: HashMap<&'a str, u32>,
    pub variables: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub macros: HashMap<Cow<'a, str>, Macros<'a>>,
    pub pc: u32,
    pub source: &'a str,
    pub section: Section,
    pub entry: Option<&'a str>,
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

    pub fn define_directive(&mut self, name: &'a str, value: Function<'a>) {
        let name = Cow::Borrowed(name);
        self.macros.insert(name, Macros::Directive(value));
    }

    pub fn define_submacro(&mut self, name: &'a str, value: SubMacro<'a>) {
        let name = Cow::Borrowed(name);
        self.macros.insert(name, Macros::Substitution(value));
    }

    pub fn content(&self) -> &str {
        // let len = self.lex.pos();
        // self.source
        // .get(self.lex.token_start..len)
        // .ok_or(ParserErrorKind::Eof)
        // .unwrap()
        todo!()
    }

    fn handle_macro(&mut self) -> Symbol<'_> {
        // self.lex.advance_while(|x| x.is_alphabetic());
        // let macro_name = self.content().to_string();

        // let macro_lookup = self.macros.get(macro_name.trim_start_matches("."));

        // let macro_body = if macro_name == ".macro" {
        //     self.lex.advance_untill(".endm");
        //     let len = self.lex.pos();
        //     self.source
        //         .get(self.lex.token_start..len)
        //         .unwrap() // TODO: fix this with error
        //         .lines()
        //         .collect::<Vec<_>>()
        // } else {
        //     self.lex.advance_untill("\n");
        //     let len = self.lex.pos();
        //     self.source
        //         .get(self.lex.token_start..len)
        //         .unwrap()
        //         .split_whitespace()
        //         .skip(1)
        //         .collect::<Vec<_>>()
        // };

        // let lookup_res = match macro_lookup {
        //     Some(Macros::Directive(pred)) => pred(self, macro_body),
        //     Some(Macros::Substitution(sub)) => {
        //         assert_eq!(sub.input.len(), macro_body.len());

        //         // let mut resolved = sub.body.join(" ");

        //         // for (&param, &value) in sub.input.iter().zip(macro_body.iter()) {
        //         //     resolved = resolved.replace(param, value);
        //         // }

        //         let resolved = sub
        //             .body
        //             .iter()
        //             .map(|&word| {
        //                 let mut result = word;
        //                 for (&param, &value) in sub.input.iter().zip(macro_body.iter()) {
        //                     if word == param {
        //                         result = value;
        //                     }
        //                 }
        //                 result
        //             })
        //             .collect::<Vec<_>>();

        //         println!("{resolved:?}");
        //         // Ok(Some(
        //         //     resolved
        //         //         .split_whitespace()
        //         //         .map(|x| x.trim())
        //         //         .collect::<Vec<_>>(),
        //         // ))
        //         todo!()
        //     }
        //     _ => todo!("{}", macro_name),
        // }
        // .map_err(|x| println!("{x}"))
        // .unwrap();

        // Symbol::Macro
        todo!()
    }

    pub fn token(&mut self) -> Token<'_> {
        // let lexeme = self.content();
        // let offset = self.lex.token_start;
        // let line = self.lex.line;

        // Token::from_str(lexeme, offset, line)
        todo!()
    }

    pub fn comment(&mut self) -> Symbol<'_> {
        // self.lex.advance_while(|x| x != '\n');

        // Symbol::Comment(self.token())
        todo!()
    }

    // pub fn tokenize(&'a mut self) -> impl Iterator<Item = Symbol> + 'a {
    //     std::iter::from_fn(|| {
    //         let tokens = self.advance_token();
    //         if tokens != Symbol::Eof {
    //             Some(tokens)
    //         } else {
    //             None
    //         }
    //     })
    // }

    pub fn is_eof(&self) -> bool {
        // self.lex.is_eof()
        true
    }

    pub fn handle(&'a mut self) -> Res<()> {
        for s in Scanner::new(self.source).tokenize() {
            println!("s: {s:#?}");
        }

        Ok(())
    }

    pub fn advance_token(&mut self) -> Symbol<'_> {
        // self.lex.advance_while(|x| x.is_whitespace());
        // self.lex.token_start = self.lex.pos();

        // let c = match self.lex.advance() {
        //     Some(c) => c,
        //     None => return Symbol::Eof,
        // };
        let c = 'x';

        match c {
            '.' => self.handle_macro(),
            ';' => self.comment(),
            x if x.is_alphabetic() => {
                // self.lex.advance_while(|x| x != ' ');
                self.handle_macro()
                // todo!()
            }
            _ => panic!(
                // "{}",
                // EsiuxErrorKind::UnknownSymbol(c, self.lex.line, self.lex.pos())
            ),
        }
    }
}
