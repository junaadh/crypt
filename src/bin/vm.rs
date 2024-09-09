use esiux_isa::{
    machine::{halt, Cpu},
    parser::ToNum,
    processor::Instruction,
    Res,
};

fn main() -> Res<()> {
    let mut vm = Cpu::default();
    vm.define_interrupt(0xf0, halt);

    vm.load_program(&program()?, 0)?;

    vm.execute()?;

    Ok(())
}

fn program() -> Res<Vec<u8>> {
    let a = "mov r1, #3";
    let b = "mov r2, #5";
    let c = "add r0, r1, r2";
    let d = "svc #0xf0";

    let a = a.parse::<Instruction>()?;
    let b = b.parse::<Instruction>()?;
    let c = c.parse::<Instruction>()?;
    let d = d.parse::<Instruction>()?;

    let mut vec = Vec::new();

    let a = a.mask();
    let b = b.mask();
    let c = c.mask();
    let d = d.mask();

    vec.extend_from_slice(&a.to_le_bytes());
    vec.extend_from_slice(&b.to_le_bytes());
    vec.extend_from_slice(&c.to_le_bytes());
    vec.extend_from_slice(&d.to_le_bytes());

    Ok(vec)
}
