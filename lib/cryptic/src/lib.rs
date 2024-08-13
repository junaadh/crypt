pub mod error;
pub mod mem;
pub mod register;
pub mod vm;

/// internal result type alias
/// each main type will have a pub(super) Res<T> type alias
/// where E: Error type for specific module
pub(crate) type R<T, E> = Result<T, E>;

/// main res type exposed publical;y
pub type Res<T> = R<T, error::Cryperror>;
