use std::str::FromStr;

use crate::{error::EsiuxErrorKind, processor::Register, Res};

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

impl l12 {
    pub fn new_u(value: u16) -> Res<l12> {
        if value > 0xfff {
            Err(EsiuxErrorKind::Overflow12(value))
        } else {
            Ok(l12 { value })
        }
    }

    pub fn new_i(value: i16) -> Res<l12> {
        if value > 0 {
            Self::new_u(value.unsigned_abs())
        } else {
            let value = value.unsigned_abs();
            let ones_compl = !(value & 0xfff);
            Self::new_u((ones_compl + 1) & 0xfff)
        }
    }

    pub fn from_str_radix(s: &str, base: u32) -> Res<l12> {
        match u16::from_str_radix(s, base) {
            Ok(u) => Self::new_u(u),
            Err(_) => match i16::from_str_radix(s, base) {
                Ok(i) => Self::new_i(i),
                Err(err) => Err(err.into()),
            },
        }
    }

    pub fn as_signed(&self) -> i16 {
        let sign = ((self.value & 0x0800) >> 11) == 0;
        if sign {
            (self.value & 0xfff) as i16
        } else {
            (self.value | 0xf000) as i16
        }
    }
}

impl FromStr for l12 {
    type Err = EsiuxErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (base, offset) = if s.len() >= 3 {
            match s {
                x if &x[..2] == "0x" => (16, 2usize),
                x if &x[..2] == "0b" => (2, 2usize),
                _ => (10, 0usize),
            }
        } else {
            (10, 0usize)
        };

        Self::from_str_radix(&s[offset..], base)
    }
}

/// # Literal 24 bit
///
/// * Representation of imm values as 24 bits
/// * We can represent both signed and unsigned values
/// * since L24 is a wrapper around u16 we have 24 usable bits
/// * will have dedicated functions to get either signed or unsigned representation
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct l20 {
    pub value: u32,
}

impl_pretty_print!(l20);

impl l20 {
    pub fn new_u(value: u32) -> Res<l20> {
        if value > 0xfffff {
            Err(EsiuxErrorKind::Overflow20(value))
        } else {
            Ok(l20 { value })
        }
    }

    pub fn new_i(value: i32) -> Res<l20> {
        if value > 0 {
            Self::new_u(value.unsigned_abs())
        } else {
            let value = value.unsigned_abs();
            let ones_compl = !(value & 0xfffff);
            Self::new_u((ones_compl + 1) & 0xfffff)
        }
    }

    pub fn from_str_radix(s: &str, base: u32) -> Res<l20> {
        match u32::from_str_radix(s, base) {
            Ok(u) => Self::new_u(u),
            Err(_) => match i32::from_str_radix(s, base) {
                Ok(i) => Self::new_i(i),
                Err(err) => Err(err.into()),
            },
        }
    }

    pub fn as_signed(&self) -> i32 {
        let sign = ((self.value & 0x080000) >> 19) == 0;
        if sign {
            (self.value & 0xfffff) as i32
        } else {
            (self.value | 0xfff00000) as i32
        }
    }
}

impl FromStr for l20 {
    type Err = EsiuxErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (base, offset) = if s.len() >= 3 {
            match s {
                x if &x[..2] == "0x" => (16, 2usize),
                x if &x[..2] == "0b" => (2, 2usize),
                _ => (10, 0usize),
            }
        } else {
            (10, 0usize)
        };

        Self::from_str_radix(&s[offset..], base)
    }
}

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

impl FromStr for Operand {
    type Err = EsiuxErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..1] {
            "#" => {
                let imm = &s[1..];
                let imm = imm.parse::<l12>()?;
                Ok(Self::Imm(imm))
            }
            "r" => {
                let reg = s.parse::<Register>()?;
                Ok(Self::Reg(reg))
            }
            _ => Err(Self::Err::FromStr(Box::new(s.to_owned()))),
        }
    }
}
