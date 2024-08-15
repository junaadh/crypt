use cryptic_derive::Codable;

#[derive(Debug, PartialEq, PartialOrd, Codable)]
#[repr(u8)]
pub enum Op {
    #[code(0x1, "nop")]
    Nop,
    #[code(0x2, "push")]
    Push(u8),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn op_one() -> crate::Res<()> {
        let nop = Code::Nop;
        let test = Code::try_from(0x1u8)?;
        assert_eq!(nop, test);
        Ok(())
    }

    #[test]
    fn op_two() -> crate::Res<()> {
        let nop = Code::Nop;
        let test: Code = 0x1u8.try_into()?;
        assert_eq!(nop, test);
        Ok(())
    }

    #[test]
    fn op_three() -> crate::Res<()> {
        let nop = Code::Nop;
        let test = "nop".parse::<Code>()?;
        assert_eq!(nop, test);
        Ok(())
    }
}
