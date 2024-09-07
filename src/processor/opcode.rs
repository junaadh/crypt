use emacro::Codable;

use super::{BRI, DPI, LSI, SCI};

/// # Instruction
///
/// DPI = 0b001 = 0x1
/// LsI = 0b011 = 0x3
/// BRI = 0b101 = 0x5
/// SCI = 0b111 = 0x7
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Codable)]
#[error(crate::error::EsiuxErrorKind)]
pub enum Instruction {
    #[alias("add", 0x11)]
    Add(DPI),
    #[alias("sub", 0x12)]
    Sub(DPI),
    #[alias("mul", 0x13)]
    Mul(DPI),
    #[alias("div", 0x14)]
    Div(DPI),
    #[alias("mov", 0x15)]
    Mov(DPI),
    #[alias("and", 0x16)]
    And(DPI),
    #[alias("orr", 0x17)]
    Or(DPI),
    #[alias("lsl", 0x18)]
    Lsl(DPI),
    #[alias("lsr", 0x19)]
    Lsr(DPI),

    #[alias("ldr", 0x30)]
    Ldr(LSI),
    #[alias("str", 0x31)]
    Str(LSI),

    #[alias("b", 0x51)]
    Branch(BRI),

    #[alias("svc", 0x71)]
    Svc(SCI),
}
