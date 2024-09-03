use std::{fmt, str::FromStr};

use cryptic_derive::Codable;

use crate::{error, register::Register};

#[derive(Debug, PartialEq, PartialOrd, Codable)]
#[repr(u8)]
pub enum Op {
    #[code(0xdf, "nop")]
    Nop,
    #[code(0x10, "push")]
    Push(Register),
    #[code(0x11, "pop")]
    Pop(Register),

    #[code(0x20, "ldr")]
    Load(Register, Operand),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operand {
    Immediate(u32),
    Register(Register),
    Offset(Register, Box<Operand>),
}

// impl FromStr for Operand {
//     type Err = error::Cryperror;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match &s[0..1] {
//             "#" | "=" =>
//         }
//     }
// }

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immediate(v) => write!(f, "{v}"),
            Self::Register(r) => write!(f, "{r}"),
            Self::Offset(r, o) => write!(f, "[{r}, {o}]"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn op_one() -> crate::Res<()> {
        let nop = Code::Nop;
        let test = Code::try_from(0xdfu8)?;
        assert_eq!(nop, test);
        Ok(())
    }

    #[test]
    fn op_two() -> crate::Res<()> {
        let nop = Code::Nop;
        let test: Code = 0xdfu8.try_into()?;
        assert_eq!(nop, test);
        Ok(())
    }

    #[test]
    fn op_three() -> crate::Res<()> {
        let nop = Code::Nop;
        let test = "nop".parse::<Code>()?;
        assert_eq!(nop, test);
        Ok(())
    }
}
