use std::{env, fs, io::Read, process};

use esiux_isa::{assembly::PreProcessor, Res};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage:\n\t{} <file_.asm>", args[0]);
        process::exit(1);
    }

    // let mut assembler = Assembler::new(&args[1])?;
    // assembler.collect_labels();

    // let program = assembler.assemble()?;

    // std::io::stdout().write_all(&program)?;
    let mut buf = String::new();
    let mut f = fs::File::open(&args[1])?;
    f.read_to_string(&mut buf)?;

    let mut pp = PreProcessor::new(buf.as_str());

    pp.handle()?;
    // println!("{pp:#?}");

    // stdout().write_all(&slice)?;

    Ok(())
}
