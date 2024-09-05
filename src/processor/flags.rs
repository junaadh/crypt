/// # Condition Flags
///
/// * 4 bits at the front of each instruction
/// * These correspond to the value of CPSR register
/// * CPSR contains 4 main flags
///   * N - Negative Flag
///   * Z - Zero Flag
///   * C - Carry Flag
///   * V - Overflow Flag
pub enum Condtion {
    /// ## Equal
    /// * execute if Z flag set
    /// * result is zero
    /// ```rust
    /// 0b0000
    /// ```
    Eq,
    /// ## Not Equal
    /// * Execute if Z flag not set
    /// ```rust
    /// 0b0001
    /// ```
    Ne,
    /// ## Carry Set / Unsigned Higher
    /// * execute if C flags set
    /// ```rust
    /// 0b0010
    /// ```
    Cs,
    /// ## Carry Clear / Unsigned lower
    /// * execute if C flag clear
    /// ```rust
    /// 0b0011
    /// ```
    Cc,
    /// ## Minus / Negetive
    /// * Execute if N flag set
    /// ```rust
    /// 0b0100
    /// ```
    Mi,
    /// ## Plus / Positive
    /// * Execute if N flag clear
    /// ```rust
    /// 0b0101
    /// ```
    Pl,
    /// ## Overflow set
    /// * Execute if V flag set
    /// ```rust
    /// 0b0110
    /// ```
    Vs,
    /// ## Overflow clear
    /// * Execute if V flag clear
    /// ```rust
    /// 0b0111
    /// ```
    Vc,
    /// ## Unsigned higher
    /// * Execute if both C and Z flag set
    /// ```rust
    /// 0b1000
    /// ```
    Hi,
    /// ## Unsigned lower
    /// * Execute if C is clear and Z flag set
    /// ```rust
    /// 0b1001
    /// ```
    Ls,
    /// ## Greater than or Equal
    /// * Execute if N == V
    /// ```rust
    /// 0b1010
    /// ```
    Ge,
    /// ## Less than
    /// * Execute if N != V
    /// ```rust
    /// 0b1011
    /// ```
    Lt,
    /// ## Greater than
    /// * Execute if Z == 0 and N == V
    /// ```rust
    /// 0b1100
    /// ```
    Gt,
    /// ## Less than or Equal
    /// * Execute if Z == 1 or N != V
    /// ```rust
    /// 0b1101
    /// ```
    Le,
    /// ## Always
    /// * Execute always
    /// ```rust
    /// 0b1110
    /// ```
    Al,
    /// ## Never
    /// * Never execute
    /// ```rust
    /// 0b1111
    /// ```
    Nv,
}

/// # CPSR Flags
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CPSRflags {
    /// * Set when the result of an operation is negative
    /// * based on the sign bit of the result
    pub(crate) n: bool,
    /// * Set when the result of an operation is zero
    pub(crate) z: bool,
    /// * Set when an operation results in a carry out from the most significant bit for unsigned operations
    /// * used for addition, subtraction, and shifts
    pub(crate) c: bool,
    /// * Set when the result of an operation causes a signed overflow
    /// * when the result doesn’t fit in the signed range of the number
    pub(crate) v: bool,
}
