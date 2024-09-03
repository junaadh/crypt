use cryptic_derive::Cryptee;

#[derive(Cryptee)]
pub enum MemError {
    ///memory out of bounds: 0x{:02x} ; {:032b}
    OutOfBounds(u32),
    ///failed to read memory address: 0x{:02x} ; {:032b}
    MemRead(u32),
}
