use cryptic_derive::Cryptee;

#[derive(Cryptee)]
pub enum VmError {
    /// vm interrupted
    Interrupt,
}
