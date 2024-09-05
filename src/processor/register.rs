use emacro::EnumFrom;

pub const REGISTER_NO: usize = 16;

/// # Registers
///
/// * 17 registers based on arm architecture
/// * 12 general purpose registers
/// * one stack pointer register
/// * one link register
/// * one program counter
/// * and one current program status register (Flags) - moved to a seperate struct
///
/// * WIP: One register will be allocated for sys call args
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumFrom)]
#[error(crate::error::EsiuxErrorKind)]
pub enum Register {
    #[code("rzr", "RZR", "r0", "R0")]
    R0, // R0
    #[code("r1", "R1")]
    R1,
    #[code("r2", "R2")]
    R2,
    #[code("r3", "R3")]
    R3,
    #[code("r4", "R4")]
    R4,
    #[code("r5", "R5")]
    R5,
    #[code("r6", "R6")]
    R6,
    #[code("r7", "R7")]
    R7,
    #[code("r8", "R8")]
    R8,
    #[code("r9", "R9")]
    R9,
    #[code("r10", "R10")]
    R10,
    #[code("r11", "R11")]
    R11,
    #[code("r12", "R12")]
    R12,
    #[code("r13", "R13", "sp", "SP")]
    SP, // R13
    #[code("r14", "R14", "lr", "LR")]
    LR, // R14
    #[code("r15", "R15", "pc", "PC")]
    PC, // R15
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reg_one() -> crate::Res<()> {
        let reg = Register::R0;
        let test: Register = 0_u8.try_into()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_two() -> crate::Res<()> {
        let reg = Register::R0;
        let test: Register = 0_u16.try_into()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_three() -> crate::Res<()> {
        let reg = Register::R0;
        let test: Register = "rzr".parse()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_four() -> crate::Res<()> {
        let reg = Register::R0;
        let test = "R0".parse::<Register>()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_five() -> crate::Res<()> {
        let reg = Register::R0;
        let test = Register::try_from(0_u8)?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_six() -> crate::Res<()> {
        let reg = Register::R0;
        let test = Register::try_from(0_u16)?;
        assert_eq!(reg, test);
        Ok(())
    }
}
