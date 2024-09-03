mod halt;

use crate::{vm::CVMachine, Res};
pub use halt::*;

pub type Interrupt = fn(&mut CVMachine, u32) -> Res<()>;

pub trait InterruptHandler {
    fn handle(&self, vm: &mut CVMachine, args: u32) -> Res<()>;
}

impl<F> InterruptHandler for F
where
    F: Fn(&mut CVMachine, u32) -> Res<()>,
{
    fn handle(&self, vm: &mut CVMachine, args: u32) -> Res<()> {
        (self)(vm, args)
    }
}
