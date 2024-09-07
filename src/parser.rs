use std::{fmt::Display, str::FromStr};

use crate::processor::{Instruction, Op, Register};

pub trait ToNum {
    fn mask(&self) -> u32;
}

impl ToNum for bool {
    fn mask(&self) -> u32 {
        if *self {
            1
        } else {
            0
        }
    }
}

impl FromStr for Instruction {
    type Err = crate::error::EsiuxErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, parts) =
            s.split_once(' ')
                .ok_or(crate::error::EsiuxErrorKind::FromStr(Box::new(format!(
                    "requires a spaced delimeter: {s}"
                ))))?;
        let parts = parts.split(',').map(|x| x.trim()).collect::<Vec<&str>>();

        let instruction = instruction.parse::<Op>()?;

        match instruction {
            Op::Add => {
                // add  rd, rn, op1
                if parts.len() < 3 {
                    return Err(crate::error::EsiuxErrorKind::NotEnoughParts(
                        Box::new(instruction),
                        3,
                    ));
                }

                let rd = parts[0].parse::<crate::processor::Register>()?;
                let rn = parts[1].parse::<crate::processor::Register>()?;
                let op = parts[2].parse::<crate::types::Operand>()?;

                Ok(Self::Add(crate::processor::DPI::new(
                    instruction,
                    rd,
                    rn,
                    op,
                )))
            }
            Op::Ldr => {
                // ldr  rd, [rn]
                // ldr  rd, [rn, #4]
                // ldr  rd, [rn], #4
                todo!()
            }
            _ => Err(crate::error::EsiuxErrorKind::FromStr(Box::new(format!(
                "Failed to parse instruction: {s}"
            )))),
        }
    }
}

// impl TryFrom<u32> for Instruction {
//     type Error = crate::error::EsiuxErrorKind;

//     fn try_from(value: u32) -> Result<Self, Self::Error> {
//         let ins = ((value >> 4) & 0b111) as u8;
//         let ins = Op::try_from(((value >> 8) & 0xf) as u8 | ins << 4)?;

//         match ins {
//             Op::Add => Ok(Self::Add(DPI::try_from(value)?)),
//             _ => unreachable!(),
//         }

//         // match ins {
//         //     InsType::DPI(d) => match d.opcode {
//         //         Self::Add(d),
//         //     }
//         // }
//         // todo!()
//     }
// }

// dp = 001 : 0x1
// ld = 011 : 0x3
// br = 101 : 0x5
// sc = 111 : 0x7
