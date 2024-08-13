mod linearmem;
mod memerror;

pub use linearmem::*;
pub use memerror::*;

pub(super) type Res<T> = crate::R<T, memerror::MemError>;

pub trait Addressable {
    fn write_u8(&mut self, addr: u16, byte: u8) -> Res<()>;
    fn read_u8(&self, offset: u16) -> Res<u8>;

    fn write_u16(&mut self, addr: u16, word: u16) -> Res<()> {
        let low = word & 0xff;
        let high = word >> 8;

        self.write_u8(addr, low as u8)?;
        self.write_u8(addr + 1, high as u8)?;
        Ok(())
    }

    fn read_u16(&self, addr: u16) -> Res<u16> {
        let x0 = self.read_u8(addr)?;
        let x1 = self.read_u8(addr + 1)?;

        Ok(x0 as u16 | ((x1 as u16) << 8))
    }

    fn copy(&mut self, from: u16, to: u16, n: usize) -> Res<()> {
        for index in 0..n as u16 {
            let x = self.read_u8(from + index)?;
            self.write_u8(to + index, x)?;
        }
        Ok(())
    }
}
