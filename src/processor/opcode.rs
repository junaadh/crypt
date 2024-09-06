use emacro::Codable;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Codable)]
#[error(crate::error::EsiuxErrorKind)]
pub enum DPOp {
    #[alias("add", 1)]
    Add,
    #[alias("sub", 2)]
    Sub,
    #[alias("mul", 3)]
    Mul,
    #[alias("div", 4)]
    Div,
    #[alias("mov", 5)]
    Mov,
    #[alias("and", 6)]
    And,
    #[alias("orr", 7)]
    Or,
    #[alias("lsl", 8)]
    Lsl,
    #[alias("lsr", 9)]
    Lsr,
}
