use crate::Res;

pub trait Addressable {
    fn read_u8(&self, addr: u32) -> Res<u8>;
    fn write_u8(&mut self, addr: u32, byte: u8) -> Res<()>;

    fn read_u16(&self, addr: u32) -> Res<u16> {
        let lower = self.read_u8(addr)?;
        let upper = self.read_u8(addr + 1)?;

        Ok((lower as u16) & 0xff | (upper as u16) << 8)
    }

    fn write_u16(&mut self, addr: u32, half_word: u16) -> Res<()> {
        let lower = (half_word & 0xff) as u8;
        let higher = (half_word >> 8) as u8;

        self.write_u8(addr, lower)?;
        self.write_u8(addr + 1, higher)?;
        Ok(())
    }

    fn read_u32(&self, addr: u32) -> Res<u32> {
        let mut byte_buffer = [0u8; 4];
        for (idx, byte) in byte_buffer.iter_mut().enumerate() {
            *byte = self.read_u8(addr + idx as u32)?;
        }

        Ok(u32::from_le_bytes(byte_buffer))
    }

    fn write_u32(&mut self, addr: u32, word: u32) -> Res<()> {
        let bytes = word.to_le_bytes();
        for (idx, &byte) in bytes.iter().enumerate() {
            self.write_u8(addr + idx as u32, byte)?;
        }
        Ok(())
    }

    fn copy(&mut self, from: u32, to: u32, n: usize) -> Res<()> {
        for index in 0..n as u32 {
            let x = self.read_u8(from + index)?;
            self.write_u8(to + index, x)?;
        }
        Ok(())
    }
}
