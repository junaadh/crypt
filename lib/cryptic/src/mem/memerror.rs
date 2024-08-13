use cryptic_derive::Cryptee;

#[derive(Cryptee)]
pub enum MemError {
    ///memory out of bounds: 0x{:02x} ; {:016b}
    OutOfBounds(u16),
    ///failed to read memory address: 0x{:02x} ; {:016b}
    MemRead(u16),
}
