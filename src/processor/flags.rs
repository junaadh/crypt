use std::fmt;

use crate::parser::ToNum;

/// # Condition Flags
///
/// * 4 bits at the front of each instruction
/// * These correspond to the value of CPSR register
/// * CPSR contains 4 main flags
///   * N - Negative Flag
///   * Z - Zero Flag
///   * C - Carry Flag
///   * V - Overflow Flag
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    /// ## Equal
    /// * execute if Z flag set
    /// * result is zero
    /// ```text
    /// 0b0000
    /// ```
    Eq,
    /// ## Not Equal
    /// * Execute if Z flag not set
    /// ```text
    /// 0b0001
    /// ```
    Ne,
    /// ## Carry Set / Unsigned Higher
    /// * execute if C flags set
    /// ```text
    /// 0b0010
    /// ```
    Cs,
    /// ## Carry Clear / Unsigned lower
    /// * execute if C flag clear
    /// ```text
    /// 0b0011
    /// ```
    Cc,
    /// ## Minus / Negetive
    /// * Execute if N flag set
    /// ```text
    /// 0b0100
    /// ```
    Mi,
    /// ## Plus / Positive
    /// * Execute if N flag clear
    /// ```text
    /// 0b0101
    /// ```
    Pl,
    /// ## Overflow set
    /// * Execute if V flag set
    /// ```text
    /// 0b0110
    /// ```
    Vs,
    /// ## Overflow clear
    /// * Execute if V flag clear
    /// ```text
    /// 0b0111
    /// ```
    Vc,
    /// ## Unsigned higher
    /// * Execute if both C and Z flag set
    /// ```text
    /// 0b1000
    /// ```
    Hi,
    /// ## Unsigned lower
    /// * Execute if C is clear and Z flag set
    /// ```text
    /// 0b1001
    /// ```
    Ls,
    /// ## Greater than or Equal
    /// * Execute if N == V
    /// ```text
    /// 0b1010
    /// ```
    Ge,
    /// ## Less than
    /// * Execute if N != V
    /// ```text
    /// 0b1011
    /// ```
    Lt,
    /// ## Greater than
    /// * Execute if Z == 0 and N == V
    /// ```text
    /// 0b1100
    /// ```
    Gt,
    /// ## Less than or Equal
    /// * Execute if Z == 1 or N != V
    /// ```text
    /// 0b1101
    /// ```
    Le,
    /// ## Always
    /// * Execute always
    /// ```text
    /// 0b1110
    /// ```
    Al,
    /// ## Never
    /// * Never execute
    /// ```text
    /// 0b1111
    /// ```
    Nv,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! printer {
            ($v: expr, [ $($kind: ident),* $(,)?]) => {{
                match $v {
                    $(
                        Self::$kind => write!(f, "{}", stringify!($kind).to_lowercase()),
                    )*
                }
            }};
        }
        printer!(
            self,
            [Eq, Ne, Cs, Cc, Mi, Pl, Vs, Vc, Hi, Ls, Ge, Lt, Gt, Le, Al, Nv]
        )
    }
}

impl std::convert::TryFrom<u8> for Condition {
    type Error = crate::error::EsiuxErrorKind;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Eq),
            1 => Ok(Self::Ne),
            2 => Ok(Self::Cs),
            3 => Ok(Self::Cc),
            4 => Ok(Self::Mi),
            5 => Ok(Self::Pl),
            6 => Ok(Self::Vs),
            7 => Ok(Self::Vc),
            8 => Ok(Self::Hi),
            9 => Ok(Self::Ls),
            10 => Ok(Self::Ge),
            11 => Ok(Self::Lt),
            12 => Ok(Self::Gt),
            13 => Ok(Self::Le),
            14 => Ok(Self::Al),
            15 => Ok(Self::Nv),
            _ => Err(crate::error::EsiuxErrorKind::TryFrom(Box::new(value))),
        }
    }
}

impl std::str::FromStr for Condition {
    type Err = crate::error::EsiuxErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.trim();

        if s == "svc" {
            return Ok(Self::Al);
        }

        macro_rules! match_s {
            ($s: expr, [$($arg: ident),* $(,)?]) => {{
                match $s {
                    $(
                        x if x.ends_with(stringify!($arg).to_lowercase().as_str()) => Ok(Self::$arg),
                    )*
                    _ => Ok(Self::Al),
                }
            }};
        }
        match_s!(
            s,
            [Eq, Ne, Cs, Cc, Mi, Pl, Vs, Vc, Hi, Ls, Ge, Lt, Gt, Le, Al, Nv]
        )
    }
}

/// # CPSR Flags
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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

impl CPSRflags {
    pub fn validate(&self, cond: Condition) -> bool {
        match cond {
            Condition::Eq => self.z,
            Condition::Ne => !self.z,
            Condition::Cs => self.c,
            Condition::Cc => !self.c,
            Condition::Mi => self.n,
            Condition::Pl => !self.n,
            Condition::Vs => self.v,
            Condition::Vc => !self.v,
            Condition::Hi => self.c && !self.z,
            Condition::Ls => !self.c || self.z,
            Condition::Ge => self.n == self.v,
            Condition::Lt => self.n != self.v,
            Condition::Gt => !self.z && self.n == self.v,
            Condition::Le => self.z || self.n != self.v,
            Condition::Al => true,
            Condition::Nv => false,
        }
    }

    pub fn set_negative(&mut self, state: bool) {
        self.n = state;
    }

    pub fn set_zero(&mut self, state: bool) {
        self.z = state;
    }

    pub fn set_carry(&mut self, state: bool) {
        self.c = state;
    }

    pub fn set_overflow(&mut self, state: bool) {
        self.v = state;
    }
}

impl fmt::Display for CPSRflags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "N: {} | Z: {} | C: {} | V: {}",
            self.n.mask(),
            self.z.mask(),
            self.c.mask(),
            self.v.mask()
        )
    }
}

#[cfg(test)]
mod test {
    use super::Condition;

    #[test]
    fn condition_one() {
        let cond = "addeq";

        let condition = cond.parse::<Condition>().unwrap();

        assert_eq!(Condition::Eq, condition);
    }

    #[test]
    fn condition_two() {
        let cond = "add.eq";

        let condition = cond.parse::<Condition>().unwrap();

        assert_eq!(Condition::Eq, condition);
    }

    #[test]
    fn condition_three() {
        let cond = "add";

        let condition = cond.parse::<Condition>().unwrap();

        assert_eq!(Condition::Al, condition);
    }
}
