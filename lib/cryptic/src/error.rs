use std::fmt;

pub enum Cryperror {
    Mem(super::mem::MemError),
    VM(super::vm::VmError),
    Asm(super::asm::AsmError),
}

impl From<super::mem::MemError> for Cryperror {
    fn from(value: super::mem::MemError) -> Self {
        Self::Mem(value)
    }
}

impl From<super::vm::VmError> for Cryperror {
    fn from(value: super::vm::VmError) -> Self {
        Self::VM(value)
    }
}

impl From<super::asm::AsmError> for Cryperror {
    fn from(value: super::asm::AsmError) -> Self {
        Self::Asm(value)
    }
}

impl fmt::Debug for Cryperror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mem(x) => write!(f, "memory: {x}"),
            Self::VM(x) => write!(f, "vm: {x}"),
            Self::Asm(x) => write!(f, "assembler: {x}"),
        }
    }
}
