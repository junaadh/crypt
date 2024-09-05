use crate::Res;

pub trait ParseNumeric {
    fn parse_no<T>(&self) -> Res<T>
    where
        T: FromStrNumber;
}

impl ParseNumeric for &str {
    fn parse_no<T>(&self) -> Res<T>
    where
        T: FromStrNumber,
    {
        let (base, offset) = if self.len() >= 3 {
            match self {
                x if &x[..2] == "0x" => (16, 2usize),
                x if &x[..1] == "b" => (2, 1usize),
                _ => (10, 0usize),
            }
        } else {
            (10, 0usize)
        };
        T::from_str(&self[offset..], base)
    }
}

impl ParseNumeric for str {
    fn parse_no<T>(&self) -> Res<T>
    where
        T: FromStrNumber,
    {
        let (base, offset) = if self.len() >= 3 {
            match self {
                x if &x[..2] == "0x" => (16, 2usize),
                x if &x[..1] == "b" => (2, 1usize),
                _ => (10, 0usize),
            }
        } else {
            (10, 0usize)
        };
        T::from_str(&self[offset..], base)
    }
}

pub trait FromStrNumber: Sized {
    fn from_str(s: &str, radix: u32) -> Res<Self>;
}

macro_rules! impl_from_str_number {
    ($ty: ty) => {
        impl FromStrNumber for $ty {
            fn from_str(s: &str, radix: u32) -> Res<Self> {
                Ok(Self::from_str_radix(s, radix)?)
            }
        }
    };
}

impl_from_str_number!(u32);
impl_from_str_number!(i32);
impl_from_str_number!(u16);
impl_from_str_number!(i16);
impl_from_str_number!(u8);
impl_from_str_number!(i8);

#[cfg(test)]
mod test {
    use crate::{parse::ParseNumeric, Res};

    #[test]
    fn one() -> Res<()> {
        let ctrl = "0xabc";
        let act = 0xabc;
        let res = ctrl.parse_no::<i32>()?;
        assert_eq!(act, res);

        Ok(())
    }
}
