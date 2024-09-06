use crate::Res;

pub trait Encodable {
    fn encode(&self) -> Res<u32>;
}

pub trait Decodable {
    fn decode(instruction: u32) -> Res<Self>
    where
        Self: Sized;
}
