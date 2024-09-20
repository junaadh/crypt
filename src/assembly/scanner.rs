use core::panic;

use eparser::lexer::Lexer;

use crate::processor::get_all_op;

use super::{Symbol, SymbolStream, Token};

#[derive(Debug)]
pub struct Scanner<'a> {
    lexer: Lexer<'a>,
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lexer: Lexer::new(content),
            source: content,
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
        Token::from_str(str.trim_start_matches("."), start, self.lexer.line)
    }

    fn whitespace(&mut self) {
        self.lexer
            .advance_while(|x| matches!(x, ' ' | '\t' | '\n' | '\r'))
    }

    fn whitespace_noln(&mut self) {
        self.lexer.advance_while(|x| matches!(x, ' ' | '\t' | '\r'))
    }

    fn label(&mut self) -> SymbolStream<'a> {
        let mut st = SymbolStream(vec![Symbol::Label(Token::from_str(
            self.content(),
            self.lexer.token_start,
            self.lexer.line,
        ))]);
        if self.lexer.peek().map(|x| x == ':').expect("Non null") {
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

        if macro_name.lexeme == "macro" {
            self.lexer.advance_untill(".endm");
        } else {
            self.lexer.advance_line();
        }
        let len = self.lexer.pos();
        let body = &self.source[self.lexer.token_start..len];

        let mut macro_body = Vec::new();

        for sym in Self::new(body).tokenize() {
            macro_body.extend_from_slice(&Vec::from(sym))
        }

        Symbol::Directive(macro_name, macro_body)
    }

    fn macro_sub(&mut self) -> Symbol<'a> {
        // Symbol::Macros(macro_name, params)
        todo!()
    }

    fn literal(&mut self) -> Symbol<'a> {
        while !self.lexer.is_eof() {
            let x = self.lexer.peek();
            match x {
                Some('0'..='9' | 'a'..='f' | 'A'..='F' | 'x') => {
                    self.lexer.advance();
                    continue;
                }
                _ => break,
            }
        }

        Symbol::Literal(self.token())
    }

    fn register(&mut self) -> Symbol<'a> {
        Symbol::Register(self.token())
    }

    fn instruction(&mut self) -> SymbolStream<'a> {
        let instruction = self.token();
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
            Some(_) if instruction.lexeme.starts_with("b") => {
                self.lexer.advance_word();
                Symbol::Literal(self.token())
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
                    Symbol::Marker.into()
                } else {
                    self.directive().into()
                }
            }
            '\\' => {
                self.lexer.advance_word();
                Symbol::Param(self.token()).into()
            }
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                self.lexer.advance_word();
                let word = self.content();
                let kw = get_all_op();
                let kword = if word.contains(".") {
                    word.split_once(".").unwrap().0
                } else {
                    word
                };
                if kw.contains(kword) {
                    self.instruction()
                } else if word.contains("_")
                    || self.lexer.peek().map(|x| x == ':').expect("Label end?")
                {
                    self.label()
                } else {
                    self.macro_sub().into()
                }
            }
            _ => todo!("match c =>{c}"), //| {self:?}"),
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
}
