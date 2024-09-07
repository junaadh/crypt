pub trait ToNum {
    fn mask(&self) -> u32;
}

impl ToNum for bool {
    fn mask(&self) -> u32 {
        if *self {
            1
        } else {
            0
        }
    }
}

// impl TryFrom<u32> for Instruction {
//     type Error = crate::error::EsiuxErrorKind;

//     fn try_from(value: u32) -> Result<Self, Self::Error> {
//         let ins = ((value >> 4) & 0b111) as u8;
//         let ins = Op::try_from(((value >> 8) & 0xf) as u8 | ins << 4)?;

//         match ins {
//             Op::Add => Ok(Self::Add(DPI::try_from(value)?)),
//             _ => unreachable!(),
//         }

//         // match ins {
//         //     InsType::DPI(d) => match d.opcode {
//         //         Self::Add(d),
//         //     }
//         // }
//         // todo!()
//     }
// }

// dp = 001 : 0x1
// ld = 011 : 0x3
// br = 101 : 0x5
// sc = 111 : 0x7
