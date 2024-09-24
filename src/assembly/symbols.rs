use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol<'a> {
    Label(Token<'a>),
    Directive(Token<'a>),
    Macros(Token<'a>),
    Ident(Token<'a>),
    Instruction(Token<'a>),
    Literal(Token<'a>),
    Register(Token<'a>),
    Punct(Token<'a>),
    Param(Token<'a>),
    Input(Token<'a>),
    Whitespace(Token<'a>),
    // special cases like .endm
    Marker(Token<'a>),
    // For reconstruction purpose
    Comment(Token<'a>),
    Eof,
}

impl<'a> Symbol<'a> {
    pub fn line(&self) -> usize {
        match self {
            Self::Label(s) => s.line,
            Self::Directive(s) => s.line,
            Self::Macros(s) => s.line,
            Self::Ident(s) => s.line,
            Self::Instruction(s) => s.line,
            Self::Literal(s) => s.line,
            Self::Register(s) => s.line,
            Self::Punct(s) => s.line,
            Self::Param(s) => s.line,
            Self::Input(s) => s.line,
            Self::Whitespace(s) => s.line,
            Self::Marker(s) => s.line,
            Self::Comment(s) => s.line,
            Self::Eof => 0,
        }
    }

    pub fn lexeme(&self) -> Cow<'a, str> {
        match self {
            Self::Label(s) => s.lexeme.clone(),
            Self::Directive(s) => s.lexeme.clone(),
            Self::Macros(s) => s.lexeme.clone(),
            Self::Ident(s) => s.lexeme.clone(),
            Self::Instruction(s) => s.lexeme.clone(),
            Self::Literal(s) => s.lexeme.clone(),
            Self::Register(s) => s.lexeme.clone(),
            Self::Punct(s) => s.lexeme.clone(),
            Self::Param(s) => s.lexeme.clone(),
            Self::Input(s) => s.lexeme.clone(),
            Self::Whitespace(s) => s.lexeme.clone(),
            Self::Marker(s) => s.lexeme.clone(),
            Self::Comment(s) => s.lexeme.clone(),
            Self::Eof => Cow::Borrowed(""),
        }
    }
}

impl fmt::Display for Symbol<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Label(Token {
                lexeme,
                pc: Some(offset),
                ..
            }) => {
                write!(f, "{offset}\t; {lexeme}")
            }
            Symbol::Ident(token) => write!(f, "\t{}", token.lexeme),
            Symbol::Instruction(token) => write!(
                f,
                "{}",
                if token.lexeme.starts_with("b") {
                    format!("\t{}\t", token.lexeme)
                } else {
                    format!("\t{}", token.lexeme)
                }
            ),
            Symbol::Literal(token) => write!(f, "\t#{}", token.lexeme.trim_start_matches("#")),
            Symbol::Register(token) => write!(f, "\t{}", token.lexeme),
            Symbol::Punct(token) => write!(f, "{}", token.lexeme),
            Symbol::Param(token) => write!(f, "\\{}", token.lexeme),
            Symbol::Marker(token) => write!(f, ".{}", token.lexeme.trim_start_matches(".")),
            Symbol::Comment(token) => {
                write!(f, "; {}", token.lexeme.trim_start_matches(";").trim_start())
            }
            Symbol::Eof => write!(f, "\\eof"),
            _ => todo!("{self:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SymbolStream<'a>(pub Vec<Symbol<'a>>, pub usize);

impl<'a> SymbolStream<'a> {
    pub fn push(&mut self, sym: Symbol<'a>) {
        self.0.push(sym)
    }

    pub fn pop(&mut self) -> Option<Symbol<'_>> {
        self.0.pop()
    }

    pub fn iterate(&mut self) -> impl Iterator<Item = &Symbol<'_>> {
        self.0.iter()
    }

    pub fn extend(&mut self, sym_stream: Self) {
        self.0.extend_from_slice(&sym_stream.0)
    }
    pub fn extend_from_vec(&mut self, sym_stream: Vec<Symbol<'a>>) {
        self.0.extend_from_slice(&sym_stream)
    }
}

impl<'a> From<Symbol<'a>> for SymbolStream<'a> {
    fn from(value: Symbol<'a>) -> Self {
        Self(vec![value], 0)
    }
}

impl<'a> From<SymbolStream<'a>> for Vec<Symbol<'a>> {
    fn from(value: SymbolStream<'a>) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Token<'a> {
    pub lexeme: Cow<'a, str>,
    pub offset: usize,
    pub len: usize,
    pub line: usize,
    pub pc: Option<usize>,
}

impl<'a> Token<'a> {
    pub fn new(
        content: &'a str,
        offset: usize,
        len: usize,
        line: usize,
        pc: Option<usize>,
    ) -> Token<'a> {
        Self {
            lexeme: Cow::Borrowed(content),
            offset,
            len,
            line,
            pc,
        }
    }

    pub fn from_str(content: &'a str, offset: usize, line: usize, pc: Option<usize>) -> Token<'a> {
        Self {
            lexeme: Cow::Borrowed(content),
            offset,
            len: content.len(),
            line,
            pc,
        }
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            lexeme: Cow::Borrowed(value),
            ..Default::default()
        }
    }
}

impl<'a> From<String> for Token<'a> {
    fn from(value: String) -> Self {
        Self {
            lexeme: Cow::Owned(value),
            ..Default::default()
        }
    }
}

impl<'a> From<Cow<'a, str>> for Token<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self {
            lexeme: value,
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! sym {
    (# $tt: expr) => {{
        use $crate::assembly::{Symbol as S, Token as T};
        S::Literal(T::from($tt))
    }};

    (reg $tt: expr) => {{
        use $crate::assembly::{Symbol as S, Token as T};
        S::Register(T::from($tt))
    }};

    (op $tt: expr) => {{
        use $crate::assembly::{Symbol as S, Token as T};
        S::Instruction(T::from($tt))
    }};

    (; $tt: expr) => {{
        use $crate::assembly::{Symbol as S, Token as T};
        S::Comment(T::from($tt))
    }};

    (;; $tt: tt) => {{
        use $crate::assembly::{Symbol as S, Token as T};
        S::Punct(T::from($tt))
    }};

    (@sort $stream: expr, $tt: expr) => {{
        $stream.push(match &$tt[..1] {
            "#" => sym!(# $tt),
            "r" | "R" => sym!(reg $tt),
            ";" => sym!(; $tt),
            ":" | "," => sym!(;; $tt),
            _ => sym!(op $tt),
        })
    }};

    ($($tt: expr),* $(,)?) => {{
        let mut stream = $crate::assembly::SymbolStream::default();
        $(
            sym!(@sort stream, $tt);
        )*
        stream
    }};
}

#[cfg(test)]
mod test {
    use crate::assembly::{Symbol, SymbolStream};

    use super::Token;

    #[test]
    fn sym_one() {
        let sym = sym!(# "#20");
        let ctrl = Symbol::Literal(Token::from("#20"));

        assert_eq!(sym, ctrl);
    }

    #[test]
    fn sym_two() {
        let sym = sym!(reg "R1");
        let ctrl = Symbol::Register(Token::from("R1"));

        assert_eq!(sym, ctrl);
    }

    #[test]
    fn sym_three() {
        let sym = sym!(op "Add");
        let ctrl = Symbol::Instruction(Token::from("Add"));

        assert_eq!(sym, ctrl);
    }

    #[test]
    fn sym_four() {
        let sym = sym!(; "; Fuck");
        let ctrl = Symbol::Comment(Token::from("; Fuck"));

        assert_eq!(sym, ctrl);
    }

    #[test]
    fn sym_five() {
        let mut st = SymbolStream::default();
        sym!(@sort st, "b.eq");
        let ctrl = Symbol::Instruction(Token::from("b.eq"));

        assert_eq!(st, ctrl.into());
    }

    #[test]
    fn sym_six() {
        let sym = sym! {
            "mov", "r1", ",", "#0",
            "svc", "0xf0"
        };
        println!("{sym:#?}");
    }
}
