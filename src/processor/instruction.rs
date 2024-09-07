use std::fmt;

use crate::{
    parser::ToNum,
    types::{l12, l24, Operand},
};

use super::{Condition, Op, Register};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct DPI {
    // 4 bits
    pub cond: Condition,
    pub instruction_type: u8,
    pub imm: bool,
    pub opcode: Op,
    pub rn: Register,
    pub rd: Register,
    pub operand: Operand,
}

impl fmt::Display for DPI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operand {
            Operand::Imm(imm) => write!(
                f,
                "{}  {}, {}, {}\t; {:02x}",
                self.opcode, self.rd, self.rn, imm, imm
            ),
            Operand::Reg(r) => write!(f, "{}  {}, {}, {}", self.opcode, self.rd, self.rn, r,),
        }
    }
}

impl ToNum for DPI {
    fn mask(&self) -> u32 {
        let mut mask = self.cond as u32;
        mask |= (((self.opcode as u32) >> 4) & 0b111) << 4;
        // mask |= ((self.instruction_type as u32) & 0b11) << 4;
        if self.imm {
            mask |= 1 << 7;
        } else {
            mask |= 0 << 7;
        }
        mask |= ((self.opcode as u32) & 0xf) << 8;
        mask |= ((self.rd as u32) & 0xf) << 12;
        mask |= ((self.rn as u32) & 0xf) << 16;
        match self.operand {
            Operand::Reg(r) => mask |= ((r as u32) & 0xf) << 20,
            Operand::Imm(i) => mask |= ((i.value as u32) & 0xfff) << 20,
        }

        mask
    }
}

impl TryFrom<u32> for DPI {
    type Error = crate::error::EsiuxErrorKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let cond = Condition::try_from((value as u8) & 0xf)?;
        let ins = ((value >> 4) & 0b111) as u8;
        let imm = ((value >> 7) & 0b1) == 1;
        let opcode = Op::try_from(((value >> 8) & 0xf) as u8 | ins << 4)?;
        let rd = Register::try_from(((value >> 12) & 0xf) as u8)?;
        let rn = Register::try_from(((value >> 16) & 0xf) as u8)?;
        let operand = match imm {
            true => Operand::Imm(l12 {
                value: (((value >> 20) & 0xfff) as u16),
            }),
            false => Operand::Reg(Register::try_from(((value >> 20) & 0xf) as u8)?),
        };

        Ok(DPI {
            cond,
            instruction_type: ins,
            imm,
            opcode,
            rn,
            rd,
            operand,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct LSI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub index: bool,
    pub negative: bool,
    pub write_back: bool,
    pub load_store: Op,
    pub rd: Register,
    pub rn: Register,
    pub offset: l12,
}

impl fmt::Display for LSI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.index {
            true => write!(
                f,
                "{}  {}, [{}, {}]",
                self.load_store, self.rd, self.rn, self.offset
            ),
            false => write!(
                f,
                "{} {}, [{}], {}",
                self.load_store, self.rd, self.rn, self.offset
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BRI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub opcode: Op,
    // TODO: support labels
    pub offset: l24,
}

impl fmt::Display for BRI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  {}", self.opcode, self.offset)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SCI {
    pub cond: Condition,
    pub instruction_type: Op,
    pub interrupt_key: u8,
    _padding: u16,
}

impl fmt::Display for SCI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "mov  {}, {}", Register::R8, self.interrupt_key)?;
        write!(f, "{}  {}", self.instruction_type, self.interrupt_key)
    }
}

#[cfg(test)]
mod test {
    use crate::{parser::ToNum, processor::Instruction};

    use super::DPI;

    #[test]
    fn one() {
        let ins = DPI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Add as u8) >> 4,
            imm: true,
            opcode: crate::processor::Op::Add,
            rn: crate::processor::Register::R2,
            rd: crate::processor::Register::R1,
            operand: crate::types::Operand::Imm(crate::types::l12 { value: 4 }),
        };

        let encoded = ins.mask();
        println!("{encoded:032b}");
        let decode = DPI::try_from(encoded).unwrap();

        assert_eq!(ins, decode)
    }
    #[test]
    fn two() {
        let ins = DPI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Add as u8) >> 4,
            imm: false,
            opcode: crate::processor::Op::Add,
            rn: crate::processor::Register::R2,
            rd: crate::processor::Register::R1,
            operand: crate::types::Operand::Reg(crate::processor::Register::R1),
        };

        let encoded = ins.mask();
        let decode = DPI::try_from(encoded).unwrap();

        assert_eq!(ins, decode)
    }

    #[test]
    fn three() {
        let ins = 0b0000_0000_0001_0000_0010_0001_0001_1110;
        let decoded = DPI::try_from(ins).unwrap();
        let repr = DPI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Add as u8) >> 4,
            imm: false,
            opcode: crate::processor::Op::Add,
            rn: crate::processor::Register::R0,
            rd: crate::processor::Register::R2,
            operand: crate::types::Operand::Reg(crate::processor::Register::R1),
        };

        let int = Instruction::try_from(ins).unwrap();
        match int {
            Instruction::Add(s) => assert_eq!(repr, s),
            _ => unreachable!(),
        }

        assert_eq!(repr, decoded)
    }

    #[test]
    fn four() {
        let ins = 0b0000_0000_0001_0000_0010_0001_1001_1110;
        let decoded = DPI::try_from(ins).unwrap();
        let repr = DPI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Add as u8) >> 4,
            imm: true,
            opcode: crate::processor::Op::Add,
            rn: crate::processor::Register::R0,
            rd: crate::processor::Register::R2,
            operand: crate::types::Operand::Imm(crate::types::l12 { value: 1 }),
        };

        assert_eq!(repr, decoded)
    }
}
