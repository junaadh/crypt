use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Section {
    Data,
    #[default]
    Text,
    Rodata,
    Bss,
    Comment,
}

impl FromStr for Section {
    type Err = crate::error::EsiuxErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim_start_matches(".") {
            "data" => Ok(Self::Data),
            "text" => Ok(Self::Text),
            "rodata" => Ok(Self::Rodata),
            "bss" => Ok(Self::Bss),
            "comment" => Ok(Self::Comment),
            _ => Err(crate::error::EsiuxErrorKind::FromStr(Box::new(
                "failed to parse segment: {s}",
            ))),
        }
    }
}

impl TryFrom<u32> for Section {
    type Error = crate::error::EsiuxErrorKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Data),
            1 => Ok(Self::Text),
            2 => Ok(Self::Rodata),
            3 => Ok(Self::Bss),
            4 => Ok(Self::Comment),
            _ => Err(crate::error::EsiuxErrorKind::TryFrom(Box::new(
                "failed to match segment: {value}",
            ))),
        }
    }
}
