pub trait Tokenizer {
    fn is_whitespace(&self) -> bool;
}

impl Tokenizer for char {
    fn is_whitespace(&self) -> bool {
        matches!(self, ' ' | '\n' | '\t')
    }
}
