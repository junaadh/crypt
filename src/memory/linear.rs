use crate::error::EsiuxErrorKind;

use super::Addressable;

pub struct LineMem {
    mem: Vec<u8>,
}

impl LineMem {
    pub fn new(size: usize) -> Self {
        Self {
            mem: vec![0u8; size],
        }
    }
}

impl Addressable for LineMem {
    fn read_u8(&self, addr: u32) -> crate::Res<u8> {
        if self.mem.len() < addr as usize {
            Err(EsiuxErrorKind::MemOutOfBounds(addr))
        } else {
            Ok(self.mem[addr as usize])
        }
    }

    fn write_u8(&mut self, addr: u32, byte: u8) -> crate::Res<()> {
        if self.mem.len() < addr as usize {
            Err(EsiuxErrorKind::MemOutOfBounds(addr))
        } else {
            self.mem[addr as usize] = byte;
            Ok(())
        }
    }
}
