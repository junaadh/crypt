mod linearmem;
mod memerror;

pub use linearmem::*;
pub use memerror::*;

pub(super) type Res<T> = crate::R<T, memerror::MemError>;

pub trait Addressable {
    fn write_u8(&mut self, addr: u32, byte: u8) -> Res<()>;
    fn read_u8(&self, offset: u32) -> Res<u8>;

    fn write_u16(&mut self, addr: u32, word: u16) -> Res<()> {
        let low = word & 0xff;
        let high = word >> 8;

        self.write_u8(addr, low as u8)?;
        self.write_u8(addr + 1, high as u8)?;
        Ok(())
    }

    fn read_u16(&self, addr: u32) -> Res<u16> {
        let x0 = self.read_u8(addr)?;
        let x1 = self.read_u8(addr + 1)?;

        Ok(x0 as u16 | ((x1 as u16) << 8))
    }

    fn write_u32(&mut self, addr: u32, dbword: u32) -> Res<()> {
        let bytes = dbword.to_le_bytes();
        for (idx, &byte) in bytes.iter().enumerate() {
            self.write_u8(addr + idx as u32, byte)?;
        }
        Ok(())
    }

    fn read_u32(&self, addr: u32) -> Res<u32> {
        let mut byte_buffer = [0u8; 4];
        for (idx, byte) in byte_buffer.iter_mut().enumerate() {
            *byte = self.read_u8(addr + idx as u32)?;
        }

        Ok(u32::from_le_bytes(byte_buffer))
    }

    fn copy(&mut self, from: u32, to: u32, n: usize) -> Res<()> {
        for index in 0..n as u32 {
            let x = self.read_u8(from + index)?;
            self.write_u8(to + index, x)?;
        }
        Ok(())
    }
}
