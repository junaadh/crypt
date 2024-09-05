use std::{fmt, str::FromStr};

use cryptic_derive::Codable;

use crate::{error, literals::Literal12, parse::ParseNumeric, register::Register, vm::VmError};

// #[derive(Debug, PartialEq, PartialOrd, Codable)]
// #[repr(u8)]
// pub enum Op {
//     #[code(0xdf, "nop")]
//     Nop,
//     #[code(0x10, "push")]
//     Push(Register),
//     #[code(0x11, "pop")]
//     Pop(Register),

//     #[code(0x20, "ldr")]
//     Load(Register, Operand),
// }

// impl FromStr for Op {
//     type Err = error::Cryperror;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if s.is_empty() {
//             return Err(error::Cryperror::VM(VmError::ParseErr(Box::new(
//                 s.to_owned(),
//             ))));
//         }
//         let (op, rest) = s.split_once(' ').unwrap_or((s, ""));
//         let op = op.parse::<Code>()?;

//         Ok(match op {
//             Code::Nop => Op::Nop,
//             Code::Push => {
//                 let reg = rest.parse::<Register>()?;
//                 Op::Push(reg)
//             }
//             Code::Pop => {
//                 let reg = rest.parse::<Register>()?;
//                 Op::Pop(reg)
//             }
//             Code::Load => {
//                 let parts = rest.split(',').map(|x| x.trim()).collect::<Vec<_>>();
//                 let reg = parts[0].parse::<Register>()?;
//                 let operand = parts[1].parse::<Operand>()?;
//                 Op::Load(reg, operand)
//             }
//             _ => unimplemented!(),
//         })
//     }
// }

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operand {
    Immediate(Literal12),
    Register(Register),
    Address(Register),
    Offset(Register, Box<Operand>),
}

impl FromStr for Operand {
    type Err = error::Cryperror;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "#" | "=" => Ok(Self::Immediate(s.parse::<Literal12>()?)),
            "r" | "R" => Ok(Self::Register(s.parse::<Register>()?)),
            "[" => {
                let inner = &s[1..s.len() - 1];
                let parts = inner.split(',').map(|x| x.trim()).collect::<Vec<_>>();
                match parts.len() {
                    1 => {
                        let reg = parts[0].parse::<Register>()?;
                        Ok(Self::Address(reg))
                    }
                    2 => {
                        let (reg, offset) = (parts[0], parts[1]);
                        let reg = reg.parse::<Register>()?;
                        let offset = match &offset[..1] {
                            "r" | "R" => Self::Register(offset.parse::<Register>()?),
                            "#" => {
                                let offset = &offset[1..];
                                Self::Immediate(offset.parse::<Literal12>()?)
                            }
                            _ => {
                                return Err(error::Cryperror::VM(VmError::ParseErr(Box::new(
                                    format!("Failed to parse :{offset}"),
                                ))))
                            }
                        };
                        Ok(Self::Offset(reg, Box::new(offset)))
                    }
                    _ => Err(error::Cryperror::VM(VmError::ParseErr(Box::new(
                        "Invalid number of parts",
                    )))),
                }
            }
            _ => Err(error::Cryperror::VM(VmError::ParseErr(Box::new(format!(
                "Failed to parse operand: {s}"
            ))))),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immediate(v) => write!(f, "{v}"),
            Self::Register(r) => write!(f, "{r}"),
            Self::Address(r) => write!(f, "[{r}]"),
            Self::Offset(r, o) => write!(f, "[{r}, {o}]"),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn op_one() -> crate::Res<()> {
//         let nop = Code::Nop;
//         let test = Code::try_from(0xdfu8)?;
//         assert_eq!(nop, test);
//         Ok(())
//     }

//     #[test]
//     fn op_two() -> crate::Res<()> {
//         let nop = Code::Nop;
//         let test: Code = 0xdfu8.try_into()?;
//         assert_eq!(nop, test);
//         Ok(())
//     }

//     #[test]
//     fn op_three() -> crate::Res<()> {
//         let nop = Code::Nop;
//         let test = "nop".parse::<Code>()?;
//         assert_eq!(nop, test);
//         Ok(())
//     }
// }
