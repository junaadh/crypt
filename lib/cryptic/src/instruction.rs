use crate::{
    // op::{Code, Operand},
    register::Register,
};

pub enum CondFlags {
    Alway = 1110,
}

/// Data Processing Instructions
///
/// ```
/// 31               19           15           11          10           03          00
///  | Literal 12 bit |  Register  |  Register  | Immediate |  operand   | Condition |
///  +----------------+------------+------------+-----------+------------+-----------+
///  | 1111 1111 1111 |    1111    |    1111    |     1     |  0011 111  |   1111    |
///
/// 31          23           19            15          11          10           03          00
///  |  Padding  |  Register  |  Register  |  Register  | Immediate |  operand   | Condition |
///  +-----------+------------+------------+------------+-----------+------------+-----------+
///  | 1111 1111 |    1111    |    1111    |    1111    |     1     |  0011 111  |   1111    |
/// ```
///
pub struct DTI {
    /// 4 bits
    pub cond: CondFlags,
    /// 7 bits : 00 start
    /// 00 00 00 0
    // pub opcode: Code,
    /// 1 bit
    pub immediate: bool,
    /// 4 bits
    pub rd: Register,
    /// 4 bits
    pub rb: Register,
    /// 12 bit imm or 4 bit reg
    // pub operand: Operand,
    /// 8 bits
    pub padding: Option<u8>,
}

/// Load / Store Instructions
///
/// ```
/// 31                    15           11           10     09      08         03          00
///  |  Operand            |  Register  | Write Back | Byte | Index |  OpCode  | Condition |
///  +---------------------+------------+------------+------+-------+----------+-----------+
///  | 1111 1111 1111 1111 |    1111    |     1      |  1   |   1   |  11111   |    1111   |
/// ```
///
pub struct LSI {
    /// 4 bits
    pub cond: CondFlags,
    /// 5 bits : 01 start
    // / 01 00 0
    // pub opcode: Code,
    /// 1 bit : true - post index | false - pre index
    pub post_index: bool,
    /// 1 bit : true - byte ( val & 0xf ) | false - word ( value & 0xffff )
    pub byte: bool,
    /// 1 byte : true - write back | false - no write
    pub write_back: bool,
    /// 4 bits
    pub rd: Register,
    /// 4 bits reg or 12 bits literal or 16 bits reg and literal
    // pub operand: Operand,
    pub padding: u8,
}
