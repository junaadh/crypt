use crate::Res;

use super::CpuCore;

pub type InterruptVector = fn(vm: &mut CpuCore, args: u32) -> Res<()>;

pub trait InterruptHandler {
    fn handle(&self, vm: &mut CpuCore, args: u32) -> Res<()>;
}

impl<F> InterruptHandler for F
where
    F: Fn(&mut CpuCore, u32) -> Res<()>,
{
    fn handle(&self, vm: &mut CpuCore, args: u32) -> Res<()> {
        self(vm, args)
    }
}

pub fn halt(vm: &mut CpuCore, args: u32) -> Res<()> {
    vm.state = false;
    println!("vm execution halted with sig: {args:02x}");
    Ok(())
}
