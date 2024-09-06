use crate::{
    parser::{Decodable, Encodable},
    types::{l12, l24, Operand},
};

use super::{Condition, DPOp, Register};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct DPI {
    // 4 bits
    pub cond: Condition,
    pub instruction_type: u8,
    pub imm: bool,
    pub opcode: DPOp,
    pub rn: Register,
    pub rd: Register,
    pub operand: Operand,
}

impl Encodable for DPI {
    fn encode(&self) -> crate::Res<u32> {
        let imm = if self.imm { 1 } else { 0u8 };
        let byte_one = ((self.cond as u8) & 0xf) << 4 | (self.instruction_type) << 2 | (imm & 0b11);
        let byte_two = ((self.opcode.as_u8()) & 0xf) << 4 | ((self.rn as u8) & 0xf);
        // let byte_three_four = ((self.rd as u8) & 0xf) | ((self.operand))
        let last12bit = match self.operand {
            Operand::Reg(r) => ((((self.rd as u8) & 0xf) << 4 | ((r as u8) & 0xf)) as u16) << 8,
            Operand::Imm(imm) => {
                let imm = imm.value & 0xfff;
                ((self.rd as u16) & 0xf) << 12 | imm
            }
        };
        let encoded = (byte_one as u32) << 24 | (byte_two as u32) << 16 | last12bit as u32;

        Ok(encoded)
    }
}

impl Decodable for DPI {
    fn decode(instruction: u32) -> crate::Res<Self> {
        let cond = ((instruction >> 28) & 0xf) as u8;
        let cond = Condition::try_from(cond)?;
        let instruction_type = ((instruction >> 26) & 0b11) as u8;
        let imm = ((instruction >> 24) & 0b11) as u8 == 1;
        let opcode = DPOp::try_from(((instruction >> 20) & 0xf) as u8)?;
        println!("Here");
        let rn = Register::try_from(((instruction >> 16) & 0xf) as u8)?;
        let rd = Register::try_from(((instruction >> 12) & 0xf) as u8)?;
        let operand = match imm {
            true => Operand::Imm(l12 {
                value: ((instruction & 0xfff) as u16),
            }),
            false => Operand::Reg(Register::try_from(((instruction >> 8) & 0xf) as u8)?),
        };
        Ok(DPI {
            cond,
            instruction_type,
            imm,
            opcode,
            rn,
            rd,
            operand,
        })
    }
}

pub struct LSI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub index: bool,
    pub negative: bool,
    pub write_back: bool,
    pub load_store: u8,
    pub rd: Register,
    pub rn: Register,
    pub offset: l12,
}

pub struct BRI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub opcode: u8,
    pub offset: l24,
}

pub struct SCI {
    pub cond: Condition,
    pub instruction_type: u8,
    pub interrupt_key: u8,
    pub rn: Register,
    _padding: u16,
}

#[cfg(test)]
mod test {
    use crate::parser::{Decodable, Encodable};

    use super::DPI;

    #[test]
    fn one() {
        let ins = DPI {
            cond: crate::processor::Condition::Al,
            instruction_type: 0b00,
            imm: true,
            opcode: crate::processor::DPOp::Add,
            rn: crate::processor::Register::R2,
            rd: crate::processor::Register::R1,
            operand: crate::types::Operand::Imm(crate::types::l12 { value: 4 }),
        };

        let encoded = ins.encode().unwrap();
        println!("{encoded:032b}");
        let decode = DPI::decode(encoded).unwrap();

        assert_eq!(ins, decode)
    }
}
