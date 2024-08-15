use std::fmt;

#[macro_export]
macro_rules! impl_try_from_str {
    (
        $(#[$attr: meta])*
        #[.error = $error_ty: ty, $err_variant: ident]
        $vis: vis enum $name: ident {
            $(
                #[$($str: expr),*]
                $variant: ident = $value: expr,
            )*
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $variant = $value,
            )*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = $error_ty;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(<$error_ty>::$err_variant(Box::new(value))),
                }
            }
        }

        impl std::convert::TryFrom<u16> for $name {
            type Error = $error_ty;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                ((value & 0xff) as u8).try_into()
            }
        }

        impl std::str::FromStr for $name {
            type Err = $error_ty;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $($str)|* => Ok(Self::$variant),
                    )*
                    _ => Err(<$error_ty>::$err_variant(Box::new(s.to_string()))),
                }
            }
        }
    };
}

impl_try_from_str! (
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    #[.error = super::vm::VmError, InvalidRegister]
    pub enum Register {
        #["rzr", "RZR", "r0", "R0"]
        RZR = 0, // R0
        #["r1", "R1"]
        R1 = 1,
        #["r2", "R2"]
        R2 = 2,
        #["r3", "R3"]
        R3 = 3,
        #["r4", "R4"]
        R4 = 4,
        #["r5", "R5"]
        R5 = 5,
        #["r6", "R6"]
        R6 = 6,
        #["r7", "R7"]
        R7 = 7,
        #["r8", "R8"]
        R8 = 8,
        #["r9", "R9"]
        R9 = 9,
        #["r10", "R10"]
        R10 = 10,
        #["r11", "R11"]
        R11 = 11,
        #["r12", "R12"]
        R12 = 12,
        #["r13", "R13", "sp", "SP"]
        SP = 13,   // R!#
        #["r14", "R14", "lr", "LR"]
        LR = 14,   // R14
        #["r15", "R15", "pc", "PC"]
        PC = 15,   // R15
        #["r16", "R16", "cpsr", "CPSR"]
        CPSR = 16, // Flags
    }
);

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RZR => write!(f, "rzr"),
            Self::R1 => write!(f, "r1"),
            Self::R2 => write!(f, "r2"),
            Self::R3 => write!(f, "r3"),
            Self::R4 => write!(f, "r4"),
            Self::R5 => write!(f, "r5"),
            Self::R6 => write!(f, "r6"),
            Self::R7 => write!(f, "r7"),
            Self::R8 => write!(f, "r8"),
            Self::R9 => write!(f, "r9"),
            Self::R10 => write!(f, "r10"),
            Self::R11 => write!(f, "r11"),
            Self::R12 => write!(f, "r12"),
            Self::SP => write!(f, "sp"),
            Self::LR => write!(f, "lr"),
            Self::PC => write!(f, "pc"),
            Self::CPSR => write!(f, "cpsr"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reg_one() -> crate::Res<()> {
        let reg = Register::RZR;
        let test: Register = 0_u8.try_into()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_two() -> crate::Res<()> {
        let reg = Register::RZR;
        let test: Register = 0_u16.try_into()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_three() -> crate::Res<()> {
        let reg = Register::RZR;
        let test: Register = "rzr".parse()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_four() -> crate::Res<()> {
        let reg = Register::RZR;
        let test = "RZR".parse::<Register>()?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_five() -> crate::Res<()> {
        let reg = Register::RZR;
        let test = Register::try_from(0_u8)?;
        assert_eq!(reg, test);
        Ok(())
    }

    #[test]
    fn reg_six() -> crate::Res<()> {
        let reg = Register::RZR;
        let test = Register::try_from(0_u16)?;
        assert_eq!(reg, test);
        Ok(())
    }
}
