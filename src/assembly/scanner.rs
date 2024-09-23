use core::panic;
use std::collections::HashMap;

use eparser::lexer::Lexer;

use crate::processor::get_all_op;

use super::{Symbol, SymbolStream, Token};

#[derive(Debug)]
pub struct Scanner<'a> {
    lexer: Lexer<'a>,
    source: &'a str,
    in_macro: bool,
    offset: usize,
    map: HashMap<&'a str, usize>,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lexer: Lexer::new(content),
            source: content,
            in_macro: false,
            offset: 0,
            map: HashMap::new(),
        }
    }

    fn content(&self) -> &'a str {
        let start = self.lexer.token_start;
        let end = self.lexer.pos();

        &self.source[start..end]
    }

    fn token(&self) -> Token<'a> {
        let start = self.lexer.token_start;
        let end = self.lexer.pos();

        let str = &self.source[start..end];
        Token::from_str(str.trim_start_matches("."), start, self.lexer.line, None)
    }

    fn pc_token(&self) -> Token<'a> {
        let start = self.lexer.token_start;
        let end = self.lexer.pos();

        let str = &self.source[start..end];
        Token::from_str(
            str.trim_start_matches("."),
            start,
            self.lexer.line,
            Some(self.offset),
        )
    }

    fn whitespace(&mut self) {
        self.lexer
            .advance_while(|x| matches!(x, ' ' | '\t' | '\n' | '\r'));
    }

    fn whitespace_noln(&mut self) {
        self.lexer.advance_while(|x| matches!(x, ' ' | '\t' | '\r'))
    }

    fn label(&mut self) -> SymbolStream<'a> {
        let mut st = SymbolStream(vec![Symbol::Label(self.pc_token())], 0);
        if self.lexer.peek().map(|x| x == ':').unwrap_or_default() {
            self.lexer.reset_ptr();
            st.push(self.punctuation(':'))
        }
        st
    }

    fn directive(&mut self) -> Symbol<'a> {
        let macro_name = {
            self.lexer.advance_word();
            self.token()
        };
        self.whitespace();
        self.lexer.reset_ptr();

        let mut macro_body = Vec::new();

        let name = if macro_name.lexeme == "macro" {
            self.lexer.advance_word();
            let decl = self.content();
            let decl_name = { Symbol::Ident(self.token()) };
            macro_body.push(decl_name);
            self.whitespace_noln();
            self.lexer.reset_ptr();
            self.lexer.advance_untill(".endm");
            decl
        } else {
            self.lexer.advance_line();
            ""
        };
        let len = self.lexer.pos();
        let body = &self.source[self.lexer.token_start..len];

        let mut pc = 0;
        for sym in {
            let mut scanner = Self::new(body);
            scanner.in_macro = true;
            scanner.tokenize_pc()
        } {
            if sym.0.is_empty() {
                pc = sym.1;
                break;
            }
            macro_body.extend_from_slice(&sym.0)
        }

        self.map.insert(name, pc); //.expect("Expected valid pc");

        Symbol::Directive(macro_name, macro_body)
    }

    fn macro_sub(&mut self) -> Symbol<'a> {
        let token = self.token();

        let mut st = SymbolStream::default();

        while self.lexer.peek().map(|x| x != '\n').unwrap() {
            self.whitespace_noln();
            self.lexer.reset_ptr();

            let op1 = match self.lexer.advance() {
                Some('r') => {
                    self.lexer.advance_word();
                    Symbol::Register(self.token())
                }
                Some('#') => {
                    self.lexer.advance_word();
                    Symbol::Literal(self.token())
                }
                Some('\\') => {
                    self.lexer.advance_word();
                    Symbol::Param(self.token())
                }
                x => panic!(
                    "Unexpected end of file: line {}: '{:?}'",
                    self.lexer.line, x
                ),
            };
            st.push(op1);
        }

        let pc_inc = self
            .map
            .get(token.lexeme.as_ref())
            .expect("Expected the offset increment");
        self.offset += pc_inc;

        // println!("{:?}", self.lexer.peek());
        // println!("{token:?}");
        // println!("{st:?}");
        Symbol::Macros(token, st.0)
    }

    fn instruction(&mut self) -> SymbolStream<'a> {
        let instruction = self.pc_token();
        self.whitespace_noln();
        self.lexer.reset_ptr();
        // println!("{:?}", self.lexer.peek());
        // println!("{:?}", instruction);
        // println!("{:?}", self);

        let op1 = match self.lexer.advance() {
            Some('r') => {
                self.lexer.advance_word();
                Symbol::Register(self.token())
            }
            Some('#') => {
                self.lexer.advance_word();
                Symbol::Literal(self.token())
            }
            Some('\\') => {
                self.lexer.advance_word();
                Symbol::Param(self.token())
            }
            Some(_) if instruction.lexeme.starts_with("b") => {
                self.lexer.advance_word();
                Symbol::Label(self.pc_token())
            }
            x => panic!(
                "Unexpected end of file: line {}: '{:?}'",
                self.lexer.line, x
            ),
        };

        let mut st = SymbolStream::default();
        st.push(Symbol::Instruction(instruction));
        st.push(op1);

        self.lexer.reset_ptr();
        if self.lexer.peek().map(|x| x == ',').unwrap() {
            let punct = self.punctuation(',');
            st.push(punct);
        }
        self.whitespace_noln();
        if self.lexer.peek().map(|x| x != '\n').unwrap() {
            self.lexer.reset_ptr();

            let op2 = match self.lexer.advance() {
                Some('r') => {
                    self.lexer.advance_word();
                    Symbol::Register(self.token())
                }
                Some('#') => {
                    self.lexer.advance_word();
                    Symbol::Literal(self.token())
                }
                Some('\\') => {
                    self.lexer.advance_word();
                    Symbol::Param(self.token())
                }
                x => panic!(
                    "Unexpected end of file: line {}: '{:?}'",
                    self.lexer.line, x
                ),
            };
            self.lexer.reset_ptr();

            let stream = if let Some(s) = self.lexer.peek() {
                let mut stream = SymbolStream::default();
                if s == ',' {
                    stream.push(self.punctuation(','));
                    self.whitespace_noln();
                    self.lexer.reset_ptr();

                    let op3 = match self.lexer.advance() {
                        Some('r') => {
                            self.lexer.advance_word();
                            Symbol::Register(self.token())
                        }
                        Some('#') => {
                            self.lexer.advance_word();
                            Symbol::Literal(self.token())
                        }
                        Some('\\') => {
                            self.lexer.advance_word();
                            Symbol::Param(self.token())
                        }
                        x => panic!(
                            "Unexpected end of file: line {}: '{:?}'",
                            self.lexer.line, x
                        ),
                    };
                    stream.push(op3)
                }
                stream
            } else {
                SymbolStream::default()
            };

            st.push(op2);
            st.extend(stream);
        }

        self.offset += 4;
        st
    }

    fn punctuation(&mut self, sym: char) -> Symbol<'a> {
        self.lexer.eat_char(sym).unwrap();
        Symbol::Punct(self.token())
    }

    fn comment(&mut self) -> SymbolStream<'a> {
        self.lexer.advance_untill("\n");
        Symbol::Comment(self.token()).into()
    }

    fn advance(&mut self) -> SymbolStream<'a> {
        self.whitespace();
        self.lexer.token_start = self.lexer.pos();

        let c = if let Some(char) = self.lexer.advance() {
            char
        } else {
            return Symbol::Eof.into();
        };

        match c {
            ';' => self.comment(),
            '.' => {
                if self.lexer.match_str("endm") {
                    self.lexer.advance_word();
                    Symbol::Marker(self.token()).into()
                } else {
                    self.directive().into()
                }
            }
            '\\' => {
                self.lexer.advance_word();
                Symbol::Input(self.token()).into()
            }
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                self.lexer.advance_word();
                let word = self.content();
                let kw = get_all_op().to_lowercase();
                let kword = if word.contains(".") {
                    word.split_once(".").unwrap().0
                } else {
                    word
                };
                if kw.contains(kword.to_lowercase().as_str()) {
                    self.instruction()
                } else if word.contains("_")
                    || self.lexer.peek().map(|x| x == ':').unwrap_or_default()
                {
                    self.label()
                } else if self.in_macro {
                    self.lexer.advance_word();
                    Symbol::Ident(self.token()).into()
                } else {
                    self.macro_sub().into()
                }
            }
            _ => todo!(
                "Unexpected character '{c}' encounter at line: {}",
                self.lexer.line
            ), //| {self:?}"),
        }
    }

    pub fn tokenize(mut self) -> impl Iterator<Item = SymbolStream<'a>> {
        std::iter::from_fn(move || {
            let token = self.advance();
            if token.0.contains(&Symbol::Eof) {
                None
            } else {
                Some(token)
            }
        })
    }

    pub fn tokenize_pc(mut self) -> impl Iterator<Item = SymbolStream<'a>> {
        std::iter::from_fn(move || {
            let token = self.advance();
            if token.0.contains(&Symbol::Eof) {
                Some(SymbolStream(Vec::new(), self.offset))
            } else {
                Some(token)
            }
        })
    }
}
