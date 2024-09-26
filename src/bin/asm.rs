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
    for st in pp.intern_buf {
        println!("{st}");
    }
    // for symbol in sym.0 {
    // match symbol {
    //     esiux_isa::assembly::Symbol::Macros(a, b)
    //     | esiux_isa::assembly::Symbol::Directive(a, b) => {
    //         println!("{a:#?}");
    //         for s in b {
    //             println!("{s:#?}");
    //         }
    //     }
    //     _ => println!("{symbol:#?}"),
    // }
    // print!("{symbol}");
    // }

    // stdout().write_all(&slice)?;

    Ok(())
}
