use std::fmt;

use crate::parser::{FromSlice, IntoSlice, Sliced};

pub const MAGIC: u32 = 0x551e0bb0; // 0xb00b1e55

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    _magic: u32,
    pub entry: u32,
    pub version: Version,
    pub section_count: u8,
    _padding: [u8; 4],
}

impl Header {
    pub fn new(entry: u32, section_count: u8) -> Self {
        Self {
            entry,
            section_count,
            ..Default::default()
        }
    }

    /// 16  bytes
    /// 128 bits
    pub fn size(&self) -> usize {
        4 + 4 + 3 + 1 + 4
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            _magic: MAGIC,
            entry: 0,
            version: Version::default(),
            section_count: 0,
            _padding: [0; 4],
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "magic: {}\nentry point: {}\nversion: {}, no of sections: {}",
            self._magic, self.entry, self.version, self.section_count,
        )
    }
}

impl FromSlice<Header> for Header {
    fn from_slice(slice: &[u8]) -> crate::Res<Header> {
        let magic = u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]);
        assert_eq!(magic, MAGIC, "Magic number is not valid");
        let entry = u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]);
        let ver = slice[8..].as_bytes::<Version>()?;
        let section = slice[11];

        Ok(Self {
            entry,
            version: ver,
            section_count: section,
            ..Default::default()
        })
    }
}

impl IntoSlice for Header {
    fn to_slice(&self) -> crate::Res<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self._magic.to_le_bytes());
        out.extend_from_slice(&self.entry.to_le_bytes());
        out.extend_from_slice(&self.version.to_slice()?);
        out.push(self.section_count);
        out.extend_from_slice(&[0; 4]);

        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u8,
    minor: u8,
    increment: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, increment: u8) -> Self {
        Self {
            major,
            minor,
            increment,
        }
    }

    pub fn increment(&mut self) {
        self.increment += 1;
    }

    pub fn minor(&mut self) {
        self.minor += 1;
        self.increment = 0;
    }

    pub fn major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.increment = 0;
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            increment: 0,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.increment)
    }
}

impl FromSlice<Version> for Version {
    fn from_slice(slice: &[u8]) -> crate::Res<Version> {
        let major = slice[0];
        let minor = slice[1];
        let increment = slice[2];
        Ok(Self::new(major, minor, increment))
    }
}

impl IntoSlice for Version {
    fn to_slice(&self) -> crate::Res<Vec<u8>> {
        Ok(vec![self.major, self.minor, self.increment])
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{IntoSlice, Sliced};

    use super::Header;

    #[test]
    fn header_one() {
        let head = Header::new(0xdeadbeef, 9);

        let head_encoded = head.to_slice().unwrap();
        let coded = [
            0xb0, 0x0b, 0x1e, 0x55, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00,
            0x00, 0x00u8,
        ];

        assert_eq!(head_encoded, coded);
    }

    #[test]
    fn header_two() {
        let head = Header::new(0xdeadbeef, 9);
        let coded = [
            0xb0, 0x0b, 0x1e, 0x55, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00,
            0x00, 0x00u8,
        ];
        let decoded = coded.as_slice().as_bytes::<Header>().unwrap();

        assert_eq!(head, decoded);
    }
}
