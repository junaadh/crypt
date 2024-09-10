use std::{env, io::Write, process};

use esiux_isa::{assembly::Assembler, Res};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage:\n\t{} <file_.asm>", args[0]);
        process::exit(1);
    }

    let mut assembler = Assembler::new(&args[1])?;
    assembler.collect_labels();

    let program = assembler.assemble()?;

    std::io::stdout().write_all(&program)?;

    Ok(())
}
