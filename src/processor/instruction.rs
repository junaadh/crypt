use std::fmt;

use crate::{
    parser::ToNum,
    types::{l12, l20, Operand},
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

impl DPI {
    pub fn new(op: Op, rd: Register, rn: Register, op1: Operand) -> Self {
        let imm = match op1 {
            Operand::Reg(_) => true,
            Operand::Imm(_) => false,
        };

        Self {
            cond: Condition::Al,
            instruction_type: ((op as u8) >> 4) & 0b111,
            imm,
            opcode: op,
            rn,
            rd,
            operand: op1,
        }
    }
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

impl ToNum for LSI {
    fn mask(&self) -> u32 {
        let mut mask = self.cond as u32;
        mask |= (((self.load_store as u32) >> 4) & 0b111) << 4;
        mask |= (self.index.mask() & 0b1) << 8;
        mask |= (self.negative.mask() & 0b1) << 9;
        mask |= (self.write_back.mask() & 0b1) << 10;
        mask |= ((self.load_store as u32) & 0b1) << 11;
        mask |= ((self.rd as u32) & 0xf) << 12;
        mask |= ((self.rn as u32) & 0xf) << 16;
        mask |= ((self.offset.value as u32) & 0xfff) << 20;

        mask
    }
}

impl TryFrom<u32> for LSI {
    type Error = crate::error::EsiuxErrorKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let cond = Condition::try_from((value & 0xf) as u8)?;
        let instruction_type = ((value >> 4) & 0b111) as u8;
        let index = ((value >> 8) & 0b1) == 1;
        let negative = ((value >> 9) & 0b1) == 1;
        let write_back = ((value >> 10) & 0b1) == 1;
        let load_store = Op::try_from(((value >> 11) & 0b1) as u8 | instruction_type << 4)?;
        let rd = Register::try_from(((value >> 12) & 0xf) as u8)?;
        let rn = Register::try_from(((value >> 16) & 0xf) as u8)?;
        let offset = l12 {
            value: ((value >> 20) & 0xfff) as u16,
        };

        Ok(Self {
            cond,
            instruction_type,
            index,
            negative,
            write_back,
            load_store,
            rd,
            rn,
            offset,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BRI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub opcode: Op,
    // TODO: support labels
    pub offset: l20,
}

impl fmt::Display for BRI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  {}", self.opcode, self.offset)
    }
}

impl ToNum for BRI {
    fn mask(&self) -> u32 {
        let mut mask = self.cond as u32;
        mask |= ((self.instruction_type as u32) & 0b111) << 4;
        mask |= ((self.opcode as u32) & 0xf) << 8;
        mask |= (self.offset.value & 0xfffff) << 12;

        mask
    }
}

impl TryFrom<u32> for BRI {
    type Error = crate::error::EsiuxErrorKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let cond = Condition::try_from((value & 0xf) as u8)?;
        let instruction_type = ((value >> 4) & 0b111) as u8;
        let opcode = Op::try_from(((value >> 8) & 0xf) as u8 | instruction_type << 4)?;
        let offset = l20 {
            value: (value >> 12) & 0xfffff,
        };

        Ok(Self {
            cond,
            instruction_type,
            opcode,
            offset,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SCI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub opcode: Op,
    pub interrupt_key: u8,
}

impl fmt::Display for SCI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  {}", self.instruction_type, self.interrupt_key)
    }
}

impl ToNum for SCI {
    fn mask(&self) -> u32 {
        let mut mask = self.cond as u32;
        mask |= ((self.instruction_type as u32) & 0b111) << 4;
        mask |= ((self.opcode as u32) & 0xf) << 8;
        mask |= ((self.interrupt_key as u32) & 0xff) << 12;

        mask
    }
}

impl TryFrom<u32> for SCI {
    type Error = crate::error::EsiuxErrorKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let cond = Condition::try_from((value & 0xf) as u8)?;
        let instruction_type = ((value >> 4) & 0b111) as u8;
        let opcode = Op::try_from(((value >> 8) & 0xf) as u8 | instruction_type << 4)?;
        let interrupt_key = ((value >> 12) & 0xff) as u8;

        Ok(Self {
            cond,
            instruction_type,
            opcode,
            interrupt_key,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        parser::ToNum,
        processor::{Instruction, SCI},
    };

    use super::{BRI, DPI, LSI};

    #[test]
    fn dpi_one() {
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
    fn dpi_two() {
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
    fn dpi_three() {
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
    fn dpi_four() {
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

    #[test]
    fn lsi_one() {
        let ins = LSI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Ldr as u8) >> 4,
            index: true,
            negative: true,
            write_back: true,
            load_store: crate::processor::Op::Ldr,
            rd: crate::processor::Register::R1,
            rn: crate::processor::Register::R2,
            offset: crate::types::l12 { value: 20 },
        };

        let encoded = ins.mask();
        let decoded: LSI = encoded.try_into().unwrap();

        assert_eq!(decoded, ins);
    }

    #[test]
    fn lsi_two() {
        let ins = LSI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Str as u8) >> 4,
            index: false,
            negative: false,
            write_back: false,
            load_store: crate::processor::Op::Str,
            rd: crate::processor::Register::R1,
            rn: crate::processor::Register::R2,
            offset: crate::types::l12 { value: 20 },
        };

        let encoded = ins.mask();
        let decoded: LSI = encoded.try_into().unwrap();

        assert_eq!(decoded, ins);
    }

    #[test]
    fn lsi_three() {
        let ins = 0b0000_0000_1111_0000_0010_0111_0011_1110;
        let decoded = LSI::try_from(ins).unwrap();
        let repr = LSI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Ldr as u8) >> 4,
            index: true,
            negative: true,
            write_back: true,
            load_store: crate::processor::Op::Ldr,
            rd: crate::processor::Register::R2,
            rn: crate::processor::Register::R0,
            offset: crate::types::l12 { value: 15 },
        };

        let int = Instruction::try_from(ins).unwrap();
        match int {
            Instruction::Ldr(s) => assert_eq!(s, decoded),
            _ => unreachable!(),
        }

        assert_eq!(repr, decoded)
    }

    #[test]
    fn lsi_four() {
        let ins = 0b0000_0000_1111_0000_0010_1000_0011_1110;
        let decoded = LSI::try_from(ins).unwrap();
        let repr = LSI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Str as u8) >> 4,
            index: false,
            negative: false,
            write_back: false,
            load_store: crate::processor::Op::Str,
            rd: crate::processor::Register::R2,
            rn: crate::processor::Register::R0,
            offset: crate::types::l12 { value: 15 },
        };

        assert_eq!(repr, decoded)
    }

    #[test]
    fn bri_one() {
        let ins = BRI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Branch as u8) >> 4,
            opcode: crate::processor::Op::Branch,
            offset: crate::types::l20 { value: 69 },
        };

        let encoded = ins.mask();
        let decoded = BRI::try_from(encoded).unwrap();

        assert_eq!(ins, decoded);
    }

    #[test]
    fn bri_two() {
        let ins = BRI {
            cond: crate::processor::Condition::Eq,
            instruction_type: (crate::processor::Op::Branch as u8) >> 4,
            opcode: crate::processor::Op::Branch,
            offset: crate::types::l20 { value: 69 },
        };

        let encoded = ins.mask();
        let decoded = BRI::try_from(encoded).unwrap();

        assert_eq!(ins, decoded);
    }

    #[test]
    fn bri_three() {
        let ins = 0b0000_0000_0000_0001_0000_0001_0101_1110;
        let decoded = BRI::try_from(ins).unwrap();

        let repr = BRI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Branch as u8) >> 4,
            opcode: crate::processor::Op::Branch,
            offset: crate::types::l20 { value: 0x10 },
        };

        let int = Instruction::try_from(ins).unwrap();
        match int {
            Instruction::Branch(b) => assert_eq!(decoded, b),
            _ => unreachable!(),
        }

        assert_eq!(repr, decoded);
    }

    #[test]
    fn bri_four() {
        let ins = 0b0000_0000_0000_0001_0000_0001_0101_1111;
        let decoded = BRI::try_from(ins).unwrap();

        let ins = BRI {
            cond: crate::processor::Condition::Nv,
            instruction_type: (crate::processor::Op::Branch as u8) >> 4,
            opcode: crate::processor::Op::Branch,
            offset: crate::types::l20 { value: 0x10 },
        };

        assert_eq!(ins, decoded);
    }

    #[test]
    fn sci_one() {
        let ins = SCI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Svc as u8) >> 4,
            opcode: crate::processor::Op::Svc,
            interrupt_key: 0xf0,
        };

        let encoded = ins.mask();
        let decoded = SCI::try_from(encoded).unwrap();

        assert_eq!(ins, decoded);
    }

    #[test]
    fn sci_two() {
        let ins = SCI {
            cond: crate::processor::Condition::Nv,
            instruction_type: (crate::processor::Op::Svc as u8) >> 4,
            opcode: crate::processor::Op::Svc,
            interrupt_key: 0xfe,
        };

        let encoded = ins.mask();
        let decoded = SCI::try_from(encoded).unwrap();

        assert_eq!(ins, decoded);
    }

    #[test]
    fn sci_three() {
        let ins = 0b0000_0000_0000_0000_1000_0001_0111_1111;

        let decoded = SCI::try_from(ins).unwrap();
        let repr = SCI {
            cond: crate::processor::Condition::Nv,
            instruction_type: (crate::processor::Op::Svc as u8) >> 4,
            opcode: crate::processor::Op::Svc,
            interrupt_key: 0x8,
        };

        let int = Instruction::try_from(ins).unwrap();
        match int {
            Instruction::Svc(s) => assert_eq!(decoded, s),
            _ => unreachable!(),
        }

        assert_eq!(decoded, repr);
    }

    #[test]
    fn sci_four() {
        let ins = 0b0000_0000_0000_0000_1001_0001_0111_1110;

        let decoded = SCI::try_from(ins).unwrap();
        let repr = SCI {
            cond: crate::processor::Condition::Al,
            instruction_type: (crate::processor::Op::Svc as u8) >> 4,
            opcode: crate::processor::Op::Svc,
            interrupt_key: 0x9,
        };

        assert_eq!(decoded, repr);
    }
}
