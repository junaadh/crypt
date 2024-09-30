use core::panic;
use std::{collections::HashMap, process};

use eparser::lexer::Lexer;

use crate::processor::{get_all_op, Op};

use super::{Statements, Symbol, Token};

#[derive(Debug)]
pub struct Scanner<'a> {
    pub(super) lexer: Lexer<'a>,
    pub(super) source: &'a str,
    pub(super) offset: u32,
    pub(super) map: HashMap<&'a str, u32>,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            lexer: Lexer::new(content),
            source: content,
            offset: 0,
            map: HashMap::new(),
        }
    }

    pub(super) fn content(&self) -> &'a str {
        let start = self.lexer.token_start;
        let end = self.lexer.pos();

        &self.source[start..end]
    }

    pub(super) fn token(&self) -> Token<'a> {
        let start = self.lexer.token_start;
        let end = self.lexer.pos();

        let str = &self.source[start..end];
        Token::from_str(str.trim_start_matches("."), start, self.lexer.line, None)
    }

    pub(super) fn pc_token(&self) -> Token<'a> {
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

    pub(super) fn whitespace(&mut self) {
        self.lexer
            .advance_while(|x| matches!(x, ' ' | '\t' | '\n' | '\r'));
    }

    pub(super) fn whitespace_noln(&mut self) {
        self.lexer.advance_while(|x| matches!(x, ' ' | '\t' | '\r'));
    }

    fn parse_operand(&mut self, branch: bool) -> Symbol<'a> {
        self.whitespace_noln();
        self.lexer.reset_ptr();
        let char = self.lexer.advance();
        self.lexer.advance_word();
        match char {
            Some('r') if !branch => Symbol::Register(self.token()),
            Some('#') if !branch => Symbol::Literal(self.token()),
            Some('\\') if !branch => Symbol::Param(self.token()),
            Some(_) if branch => Symbol::Label(self.pc_token()),
            Some('_') => Symbol::Label(self.pc_token()),
            Some(_) => Symbol::Ident(self.token()),
            x => {
                let word = {
                    self.lexer.advance_word();
                    self.content()
                };
                panic! {
                "Unexpected char: line {}: '{:?}' {}",
                self.lexer.line, x, word
                }
            }
        }
    }

    fn parse_punctuation(&mut self, char: char) {
        self.lexer
            .eat_char(char)
            .map_err(|_| {
                println!(
                    "Expected a punctuation char '{}' @ {}",
                    char, self.lexer.line
                )
            })
            .unwrap();
    }

    fn parse_instruction(&mut self) -> Statements<'a> {
        let token = self.content();
        let op = token
            .parse::<Op>()
            .map_err(|_| println!("Unrecognized instruction: {} @ {}", token, self.lexer.line))
            .unwrap();
        let masked = ((op as u8) >> 4) & 0b111;

        let instruction = Symbol::Instruction(self.token());
        if self.lexer.eat_char(' ').is_err() {
            self.lexer
                .eat_char('\t')
                .map_err(|_| println!("Expected a whitespace char ' ', '\t' @ {}", self.lexer.line))
                .unwrap();
        }

        self.offset += 4;
        match masked {
            1 => {
                let op1 = self.parse_operand(false);
                self.parse_punctuation(',');
                let op2 = self.parse_operand(false);
                if op == Op::Cmp || op == Op::Mov {
                    Statements::DPI {
                        instruction,
                        op1,
                        op2,
                        op3: None,
                    }
                } else {
                    self.parse_punctuation(',');
                    let op3 = self.parse_operand(false);
                    Statements::DPI {
                        instruction,
                        op1,
                        op2,
                        op3: Some(op3),
                    }
                }
            }
            3 => {
                let op1 = self.parse_operand(false);
                self.whitespace_noln();
                self.lexer.reset_ptr();
                let obracket = matches!(self.lexer.peek(), Some('['));

                let op2 = self.parse_operand(false);
                let mut cbracket = false;
                match self.lexer.peek() {
                    Some(',') => self.parse_punctuation(','),
                    Some(']') => cbracket = true,
                    _ => cbracket = false,
                }

                let op3 = self.parse_operand(false);

                Statements::LSI {
                    instruction,
                    op1,
                    obracket,
                    op2,
                    cbracket,
                    op3,
                }
            }
            5 => {
                let op1 = self.parse_operand(true);

                Statements::BRI {
                    instruction,
                    label: op1,
                }
            }
            7 => {
                let op1 = self.parse_operand(false);

                Statements::SCI {
                    instruction,
                    vector: op1,
                }
            }
            _ => panic!(
                "Unknown instruction type: {token} op: 0x{:02x} @ {}",
                op as u8, self.lexer.line
            ),
        }
    }

    fn parse_comment(&mut self) -> Statements<'a> {
        let token = {
            self.lexer.advance_line();
            self.token()
        };
        Statements::Comment {
            name: Symbol::Comment(token),
        }
    }

    fn parse_label(&mut self) -> Statements<'a> {
        let token = {
            self.lexer.advance_word();
            self.pc_token()
        };
        let tok = self.content();
        if let Err(e) = self.lexer.eat_char(':') {
            println!("{e}");
            process::exit(-1);
        };
        self.map.insert(tok, self.offset);
        Statements::Label {
            name: Symbol::Label(token),
        }
    }

    fn parse_directive(&mut self) -> Statements<'a> {
        let mut params = Vec::new();
        let mut body = Vec::new();
        let mut pc = 0;

        let directive = {
            self.lexer.advance_word();
            self.content()
        };
        let directive_token = self.token();
        self.whitespace_noln();
        self.lexer.reset_ptr();

        let in_macro = directive.trim_start_matches(".") == "macro";

        let mac_name = if in_macro {
            let macro_name = {
                self.lexer.advance_word();
                self.content()
            };
            let macro_name_sym = Symbol::Ident(self.token());
            params.push(macro_name_sym);
            self.whitespace_noln();
            self.lexer.reset_ptr();
            Some(macro_name)
        } else {
            None
        };

        while !self.lexer.is_eof() && self.lexer.peek().map(|x| x != '\n').unwrap_or_default() {
            let op = self.parse_operand(false);
            params.push(op);
        }

        self.whitespace_noln();
        self.lexer.reset_ptr();
        if in_macro {
            let slice = {
                self.lexer.advance_untill(".endm");
                let end = self.lexer.pos();
                self.source[self.lexer.token_start..end].trim_end_matches(".endm")
            };
            let mut inner_scanner = Self::new(slice);
            for stmt in inner_scanner.analyze() {
                match &stmt {
                    Statements::DPI { .. }
                    | Statements::LSI { .. }
                    | Statements::SCI { .. }
                    | Statements::BRI { .. } => {
                        pc += 4;
                        body.push(stmt);
                    }
                    _ => body.push(stmt),
                }
            }
        }
        let marker = if in_macro {
            Some(Symbol::Marker(Token::from(".endm")))
        } else {
            None
        };

        if in_macro {
            self.map.insert(mac_name.unwrap(), pc);
        }

        Statements::Directive {
            name: Symbol::Ident(directive_token),
            params,
            body,
            pc,
            marker,
        }
    }

    fn parse_substitution(&mut self) -> Statements<'a> {
        let token = {
            self.lexer.advance_word();
            self.token()
        };
        let mac_name = self.content();
        let mut values = Vec::new();
        while !self.lexer.is_eof() && self.lexer.peek().map(|x| x != '\n').unwrap_or_default() {
            let op = self.parse_operand(false);
            values.push(op);
        }

        let pc = self.map.get(mac_name);
        self.offset += pc.unwrap();

        Statements::Substitution {
            name: Symbol::Ident(token),
            values,
        }
    }

    pub(super) fn parse(&mut self) -> Statements<'a> {
        self.whitespace();
        self.lexer.reset_ptr();

        let c = if let Some(char) = self.lexer.advance() {
            char
        } else {
            return Statements::Eof;
        };

        match c {
            ';' => self.parse_comment(),
            '.' => self.parse_directive(),
            _ => {
                self.lexer.advance_word();
                let kw = get_all_op();
                let word = self.content();
                // TODO: handle if moveq instructions
                let s = word.split_once(".").unwrap_or((word, "")).0;
                // println!("{word}");
                if kw.contains(format!("_{}_", s).to_lowercase().as_str()) {
                    // println!("i");
                    self.parse_instruction()
                } else if word.starts_with("_")
                    || self.lexer.peek().map(|x| x == ':').unwrap_or_default()
                {
                    // println!("l");
                    self.parse_label()
                } else {
                    // println!("s");
                    self.parse_substitution()
                }
            }
        }
    }

    pub fn analyze(&mut self) -> impl Iterator<Item = Statements<'a>> + '_ {
        std::iter::from_fn(move || {
            let stmt = self.parse();
            if stmt != Statements::Eof {
                Some(stmt)
            } else {
                None
            }
        })
    }
}
