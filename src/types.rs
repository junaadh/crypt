use crate::processor::Register;

/// # Literal 12 bit
///
/// * Representation of imm values as 12 bits
/// * We can represent both signed and unsigned values
/// * since L12 is a wrapper around u16 we have 12 usable bits
/// * will have dedicated functions to get either signed or unsigned representation
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct l12 {
    pub value: u16,
}

impl_pretty_print!(l12);

/// # Literal 24 bit
///
/// * Representation of imm values as 24 bits
/// * We can represent both signed and unsigned values
/// * since L24 is a wrapper around u16 we have 24 usable bits
/// * will have dedicated functions to get either signed or unsigned representation
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct l24 {
    pub value: u32,
}

impl_pretty_print!(l24);

/// # Operand
///
/// * operand enum can be used to represent either a register or 12 bit immediate
/// * registers are represented at 4 bits
/// * immediate are represented at 12 bits
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operand {
    /// ## Register
    /// * represents a 4 bit register
    Reg(Register),
    /// ## Immediate
    /// * represents a 12 bit immediate
    Imm(l12),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(r) => write!(f, "{}", r),
            Self::Imm(imm) => write!(f, "{}", imm),
        }
    }
}
