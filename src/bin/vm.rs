use std::{
    env, fs,
    io::{self, Read},
    process,
};

use esiux_isa::{
    machine::{halt, print, Cpu},
    Res,
};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage:\n\t{} <file_.bin>", args[0]);
        process::exit(1);
    }

    let mut readable: Box<dyn Read> = match args[1].as_str() {
        "-" => Box::new(io::stdin()),
        x => Box::new(fs::File::open(x)?),
    };

    let mut vm = Cpu::default();
    vm.define_interrupt(0xf0, halt);
    vm.define_interrupt(0xe0, print);

    let mut program = Vec::<u8>::new();
    readable.read_to_end(&mut program)?;

    vm.load_program(&program, 0)?;

    vm.execute()?;

    Ok(())
}
