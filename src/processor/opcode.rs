use emacro::Codable;

use super::{BRI, DPI, LSI, SCI};

/// # Instruction
///
/// DPI = 0b001 = 0x1
/// LsI = 0b011 = 0x3
/// BRI = 0b101 = 0x5
/// SCI = 0b111 = 0x7
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Codable)]
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

#[cfg(test)]
mod test {
    use crate::{
        processor::{Condition, Op, Register, DPI, SCI},
        types::{l12, Operand},
    };

    use super::Instruction;

    #[test]
    fn op_one() {
        let ins = "add r1, r0, #1";

        let instruction = ins.parse::<Instruction>().unwrap();

        assert_eq!(
            instruction,
            Instruction::Add(DPI {
                cond: Condition::Al,
                instruction_type: 0b001,
                imm: true,
                opcode: Op::Add,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Imm(l12::new_u(1).unwrap())
            })
        );
    }

    #[test]
    fn op_two() {
        let ins = 0b0000_0000_0001_0000_0001_0001_1001_1110;

        let instruction = Instruction::try_from(ins).unwrap();

        assert_eq!(
            instruction,
            Instruction::Add(DPI {
                cond: Condition::Al,
                instruction_type: 0b001,
                imm: true,
                opcode: Op::Add,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Imm(l12::new_u(1).unwrap())
            })
        );
    }

    #[test]
    fn op_three() {
        let ins = "add r1, r0, r2";

        let instruction = ins.parse::<Instruction>().unwrap();

        assert_eq!(
            instruction,
            Instruction::Add(DPI {
                cond: Condition::Al,
                instruction_type: 0b001,
                imm: false,
                opcode: Op::Add,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Reg(Register::R2)
            })
        );
    }

    #[test]
    fn op_four() {
        let ins = 0b0000_0000_0010_0000_0001_0001_0001_1110;

        let instruction = Instruction::try_from(ins).unwrap();

        assert_eq!(
            instruction,
            Instruction::Add(DPI {
                cond: Condition::Al,
                instruction_type: 0b001,
                imm: false,
                opcode: Op::Add,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Reg(Register::R2),
            })
        );
    }

    #[test]
    fn op_five() {
        let ins = "add.eq r1, r0, r2";

        let instruction = ins.parse::<Instruction>().unwrap();

        assert_eq!(
            instruction,
            Instruction::Add(DPI {
                cond: Condition::Eq,
                instruction_type: 0b001,
                imm: false,
                opcode: Op::Add,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Reg(Register::R2)
            })
        );
    }

    #[test]
    fn op_six() {
        let ins = 0b0000_0000_0010_0000_0001_0001_0001_0000;

        let instruction = Instruction::try_from(ins).unwrap();

        assert_eq!(
            instruction,
            Instruction::Add(DPI {
                cond: Condition::Eq,
                instruction_type: 0b001,
                imm: false,
                opcode: Op::Add,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Reg(Register::R2),
            })
        );
    }

    #[test]
    fn op_seven() {
        let ins = "mov r1, #1";

        let instruction = ins.parse::<Instruction>().unwrap();

        assert_eq!(
            instruction,
            Instruction::Mov(DPI {
                cond: Condition::Al,
                instruction_type: 0b001,
                imm: true,
                opcode: Op::Mov,
                rn: Register::R0,
                rd: Register::R1,
                operand: Operand::Imm(l12::new_u(1).unwrap()),
            })
        );
    }

    #[test]
    fn op_eight() {
        let ins = "svc #0xf0";

        let instruction = ins.parse::<Instruction>().unwrap();

        assert_eq!(
            instruction,
            Instruction::Svc(SCI {
                cond: Condition::Al,
                instruction_type: 0b111,
                opcode: Op::Svc,
                interrupt_key: 0xf0
            })
        );
    }
}
