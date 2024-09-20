#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol<'a> {
    Label(Token<'a>),
    Directive(Token<'a>, Vec<Symbol<'a>>),
    Macros(Token<'a>, Vec<Symbol<'a>>),
    Instruction(Token<'a>),
    Literal(Token<'a>),
    Register(Token<'a>),
    Punct(Token<'a>),
    Param(Token<'a>),
    // special cases like .endm
    Marker,
    // For reconstruction purpose
    Comment(Token<'a>),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SymbolStream<'a>(pub Vec<Symbol<'a>>);

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
        Self(vec![value])
    }
}

impl<'a> From<SymbolStream<'a>> for Vec<Symbol<'a>> {
    fn from(value: SymbolStream<'a>) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
    pub lexeme: &'a str,
    pub offset: usize,
    pub len: usize,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(content: &'a str, offset: usize, len: usize, line: usize) -> Token<'a> {
        Self {
            lexeme: content,
            offset,
            len,
            line,
        }
    }

    pub fn from_str(content: &'a str, offset: usize, line: usize) -> Token<'a> {
        Self {
            lexeme: content,
            offset,
            len: content.len(),
            line,
        }
    }
}
