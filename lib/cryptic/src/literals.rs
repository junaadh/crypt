use std::{fmt::Display, str::FromStr};

use crate::parse::ParseNumeric;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Literal12 {
    value: u16,
}

impl FromStr for Literal12 {
    type Err = crate::error::Cryperror;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse_no::<u16>()?;
        Ok(Self { value })
    }
}

impl Display for Literal12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
