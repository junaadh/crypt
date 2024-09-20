use crate::parser::{FromSlice, IntoSlice};

use super::Section;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentHeader {
    pub size: u16,
    pub kind: Section,
    pub offset: u32,
}

impl Default for SegmentHeader {
    fn default() -> Self {
        Self {
            size: 0,
            kind: Section::default(),
            offset: 16,
        }
    }
}

impl SegmentHeader {
    pub fn new(size: u16, kind: Section, offset: u32) -> Self {
        Self { size, kind, offset }
    }
}

impl SegmentHeader {
    pub fn size() -> usize {
        2 + 2 + 4
    }
}

impl FromSlice<SegmentHeader> for SegmentHeader {
    fn from_slice(slice: &[u8]) -> crate::Res<SegmentHeader> {
        if slice.len() > 8 {
            return Err(crate::error::EsiuxErrorKind::Invalid(
                "Segment header length".to_string(),
                8,
                slice.len(),
            ));
        }

        let size = u16::from_le_bytes([slice[0], slice[1]]);
        let kind = u16::from_le_bytes([slice[2], slice[3]]);
        let offset = u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]);

        Ok(Self::new(size, kind.try_into()?, offset))
    }
}

impl IntoSlice for SegmentHeader {
    fn to_slice(&self) -> crate::Res<Vec<u8>> {
        let mut out = Vec::new();

        out.extend_from_slice(&self.size.to_le_bytes());
        out.extend_from_slice(&(self.kind as u16).to_le_bytes());
        out.extend_from_slice(&self.offset.to_le_bytes());

        Ok(out)
    }
}
