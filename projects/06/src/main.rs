mod parser;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(asm_file) = env::args().nth(1) else {
        eprintln!("No assembly file was provided.");
        return ExitCode::FAILURE;
    };

    let tokens = parser::parse_assembly(&asm_file);

    for token in tokens {
        println!("{:?}", token);
    }

    ExitCode::SUCCESS
}
