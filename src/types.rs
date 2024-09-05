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
    value: u16,
}

impl_pretty_print!(l12);
