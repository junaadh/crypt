#[derive(Debug)]
pub struct LinearMem {
    region: Vec<u8>,
}

impl LinearMem {
    pub fn new(size: usize) -> Self {
        Self {
            region: vec![0_u8; size],
        }
    }
}

impl super::Addressable for LinearMem {
    fn write_u8(&mut self, addr: u16, byte: u8) -> super::Res<()> {
        let addrv = addr as usize;
        if addrv > self.region.len() {
            return Err(super::MemError::OutOfBounds(addr));
        }

        self.region[addrv] = byte;
        Ok(())
    }

    fn read_u8(&self, offset: u16) -> super::Res<u8> {
        self.region
            .get(offset as usize)
            .copied()
            .ok_or(super::MemError::MemRead(offset))
    }
}
