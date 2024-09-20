use crate::{
    processor::{Op, Register},
    Res,
};

pub trait ToNum {
    fn mask(&self) -> u32;
}

pub trait Negative {
    fn is_negative(&self) -> bool;
}

pub trait Parser<U>: Sized {
    type Op1;

    fn parse_instruction(
        value: &str,
        opcode: Op,
        rd: Register,
        rn: Register,
        op1: Self::Op1,
    ) -> crate::Res<U>;
}

pub trait ParserImpl {
    fn mk_instruction<F: Parser<F>>(
        &self,
        opcode: Op,
        rd: Register,
        rn: Register,
        op1: F::Op1,
    ) -> crate::Res<F>;
}

impl ParserImpl for str {
    fn mk_instruction<F: Parser<F>>(
        &self,
        opcode: Op,
        rd: Register,
        rn: Register,
        op1: F::Op1,
    ) -> crate::Res<F> {
        F::parse_instruction(self, opcode, rd, rn, op1)
    }
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

pub trait FromSlice<S>: Sized {
    fn from_slice(slice: &[u8]) -> Res<S>;
}

pub trait Sliced {
    fn as_bytes<S: FromSlice<S>>(&self) -> Res<S>;
}

impl Sliced for [u8] {
    fn as_bytes<S: FromSlice<S>>(&self) -> Res<S> {
        S::from_slice(self)
    }
}

pub trait IntoSlice: Sized {
    fn to_slice(&self) -> Res<Vec<u8>>;
}
