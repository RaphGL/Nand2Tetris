mod assembler;
mod parser;

use assembler::Assembler;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(asm_file) = env::args().nth(1) else {
        eprintln!("No assembly file was provided.");
        return ExitCode::FAILURE;
    };

    let tokens = parser::parse_assembly(&asm_file);
    let mut assembler = Assembler::new(tokens);
    assembler.resolve_symbols();
    let machine_code = assembler.assemble();

    let mut hack_path = PathBuf::from(asm_file);
    hack_path.set_extension("hack");

    let mut hack_file = File::create(hack_path).unwrap();
    for inst in machine_code {
        hack_file
            .write_fmt(format_args!("{:016.b}\n", inst))
            .unwrap();
    }

    ExitCode::SUCCESS
}
