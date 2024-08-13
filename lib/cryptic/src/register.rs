use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Register {
    RZR, // R0
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,   // R!#
    LR,   // R14
    PC,   // R15
    CPSR, // Flags
}

impl TryFrom<u8> for Register {
    type Error = super::vm::VmError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Register::RZR,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::SP,
            14 => Register::LR,
            15 => Register::PC,
            16 => Register::CPSR,
            _ => return Err(super::vm::VmError::InvalidRegisterNo(value)),
        })
    }
}

impl TryFrom<u16> for Register {
    type Error = super::vm::VmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        ((value & 0xff) as u8).try_into()
    }
}

impl FromStr for Register {
    type Err = super::vm::VmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "r0" | "R0" => Register::RZR,
            // cannot do this gonna make a macro
            _ => todo!(),
        })
    }
}
