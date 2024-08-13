use cryptic::mem::MemError;

fn n() -> Result<(), MemError> {
    Err(MemError::OutOfBounds(10))
    // Err(MemError::Testing(9, 69))
}

fn main() -> cryptic::Res<()> {
    // fn main() -> Result<(), MemError> {
    n()?;
    Ok(())
}
