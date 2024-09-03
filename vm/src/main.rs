use cryptic::{mem::MemError, parse::ParseNumeric, register};

fn n() -> Result<(), MemError> {
    Err(MemError::OutOfBounds(10))
    // Err(MemError::Testing(9, 69))
}

fn main() -> cryptic::Res<()> {
    // fn main() -> Result<(), MemError> {
    // n()?;
    let mut vm = cryptic::vm::CVMachine::default();
    vm.state();
    // vm.registers[cryptic::register::Register::CPSR as usize] = ;
    vm.register(register::Register::CPSR, |_| 0x37);
    vm.register(register::Register::R0, |_| 0x69);
    let x = vm.register(register::Register::CPSR, |x| x);
    println!("{x}");

    vm.state();

    let value = 0xabc99def;
    vm.memory.write_u32(0, value)?;
    let value_read = vm.memory.read_u32(0)?;

    println!("0x{value_read:02x}");

    let ctrl = "0xabc";
    let act = 0xabc;
    let res = ctrl.parse_no::<i32>()?;

    println!("0x{res:02x}");
    println!("0x{act:02x}");

    Ok(())
}
