use crate::{
    error::EsiuxErrorKind,
    parser::{FromSlice, IntoSlice, Sliced},
};

use super::{Header, SegmentHeader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EsiuxBin {
    pub header: Header,
    pub section_headers: Vec<SegmentHeader>,
    pub data: Vec<u8>,
}

impl EsiuxBin {
    pub fn get_head_offset(&self) -> usize {
        self.header.size() + (SegmentHeader::size() + self.header.section_count as usize)
    }
}

impl FromSlice<EsiuxBin> for EsiuxBin {
    fn from_slice(slice: &[u8]) -> crate::Res<EsiuxBin> {
        let header = slice[..17].as_bytes::<Header>()?;
        let mut section_headers = Vec::new();
        for i in 0..header.section_count {
            let start = 16 + (i as usize * SegmentHeader::size());
            section_headers
                .push(slice[start..start + SegmentHeader::size()].as_bytes::<SegmentHeader>()?);
        }
        let data = slice
            .get(
                header.size() + (SegmentHeader::size() * header.section_count as usize)
                    ..slice.len(),
            )
            .ok_or(EsiuxErrorKind::EmptyBin)?
            .to_vec();

        Ok(Self {
            header,
            section_headers,
            data,
        })
    }
}

impl IntoSlice for EsiuxBin {
    fn to_slice(&self) -> crate::Res<Vec<u8>> {
        let mut out = Vec::new();

        out.extend_from_slice(&self.header.to_slice()?);
        for i in 0..self.header.section_count {
            let seg = self.section_headers[i as usize].to_slice()?;
            out.extend_from_slice(&seg);
        }
        out.extend_from_slice(&self.data);

        Ok(out)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        format::{Header, Section, SegmentHeader},
        parser::{IntoSlice, Sliced, ToNum},
        processor::Instruction,
    };

    use super::EsiuxBin;

    #[test]
    fn bin_one() {
        let data = "mov r1, #69"
            .parse::<Instruction>()
            .unwrap()
            .mask()
            .to_le_bytes()
            .to_vec();
        let bin = EsiuxBin {
            header: Header::new(0xdeadbeef, 1),
            section_headers: vec![SegmentHeader::new(4, Section::Text, 24)],
            data,
        };

        let bin_encoded = bin.to_slice().unwrap();

        let coded = [
            0xb0, 0x0b, 0x1e, 0x55, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x9e, 0x15, 0x50, 0x04u8,
        ]
        .to_vec();

        assert_eq!(coded, bin_encoded);
    }

    #[test]
    fn bin_two() {
        let data = "mov r1, #69"
            .parse::<Instruction>()
            .unwrap()
            .mask()
            .to_le_bytes()
            .to_vec();
        let bin = EsiuxBin {
            header: Header::new(0xdeadbeef, 1),
            section_headers: vec![SegmentHeader::new(4, Section::Text, 24)],
            data,
        };

        let coded = [
            0xb0, 0x0b, 0x1e, 0x55, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x9e, 0x15, 0x50, 0x04u8,
        ]
        .to_vec();

        let decoded = coded.as_slice().as_bytes::<EsiuxBin>().unwrap();

        assert_eq!(decoded, bin);
    }
}
