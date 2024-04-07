// todo: implement function, call and return instructions
use std::{env, fs, path::Path, process::ExitCode};

#[derive(Debug)]
enum SegmentAddr {
    Constant(u16),
    Static(u16),
    Temp(u16),
    Pointer(u16),
    This(u16),
    That(u16),
    Local(u16),
    Arg(u16),
}

#[derive(Debug)]
enum Inst {
    Push(SegmentAddr),
    Pop(SegmentAddr),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Label(String),
    Goto(String),
    IfGoto(String),
}

struct Parser<'a> {
    file: &'a Path,
    tokens: Vec<Inst>,
}

impl<'a> Parser<'a> {
    fn new(vm_file: &'a Path) -> Self {
        Self {
            file: vm_file,
            tokens: Vec::new(),
        }
    }

    fn parse(&mut self) -> Result<(), String> {
        let vm_code = fs::read_to_string(self.file).unwrap();

        for (linenum, line) in vm_code.lines().enumerate() {
            let line = line.trim();
            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            let line_parts: Vec<_> = line.split(' ').map(|part| part.trim()).collect();

            self.tokens.push(match line_parts[0] {
                "push" => {
                    let arg = line_parts[2].parse::<u16>().unwrap();
                    Inst::Push(match line_parts[1] {
                        "constant" => SegmentAddr::Constant(arg),
                        "static" => SegmentAddr::Static(arg),
                        "temp" => SegmentAddr::Temp(arg),
                        "pointer" => SegmentAddr::Pointer(arg),
                        "this" => SegmentAddr::This(arg),
                        "that" => SegmentAddr::That(arg),
                        "local" => SegmentAddr::Local(arg),
                        "argument" => SegmentAddr::Arg(arg),
                        invalid => {
                            return Err(format!(
                                "push on line {linenum} has invalid segment \"{invalid}\"",
                            ));
                        }
                    })
                }

                "pop" => {
                    let arg = line_parts[2].parse::<u16>().unwrap();
                    Inst::Pop(match line_parts[1] {
                        "static" => SegmentAddr::Static(arg),
                        "temp" => SegmentAddr::Temp(arg),
                        "pointer" => SegmentAddr::Pointer(arg),
                        "this" => SegmentAddr::This(arg),
                        "that" => SegmentAddr::That(arg),
                        "local" => SegmentAddr::Local(arg),
                        "argument" => SegmentAddr::Arg(arg),
                        invalid => {
                            return Err(format!(
                                "pop on line {linenum} has invalid segment \"{invalid}\"",
                            ));
                        }
                    })
                }

                "add" => Inst::Add,
                "sub" => Inst::Sub,
                "neg" => Inst::Neg,
                "eq" => Inst::Eq,
                "or" => Inst::Or,
                "and" => Inst::And,
                "not" => Inst::Not,
                "gt" => Inst::Gt,
                "lt" => Inst::Lt,

                "label" => Inst::Label(line_parts[1].into()),
                "goto" => Inst::Goto(line_parts[1].into()),
                "if-goto" => Inst::IfGoto(line_parts[1].into()),

                invalid => {
                    return Err(format!(
                        "found invalid instruction \"{invalid}\" on line {linenum}"
                    ));
                }
            })
        }

        Ok(())
    }
}

fn generate_vm_code(parser: Parser) -> String {
    let mut asm = String::from(include_str!("asm_snippets/init.asm"));
    let file = Path::new(parser.file);
    let filename = file.file_stem().unwrap().to_str().unwrap();

    for (i, inst) in parser.tokens.iter().enumerate() {
        asm.push_str(
            match inst {
                Inst::Push(push) => match push {
                    SegmentAddr::Constant(arg) => {
                        format!(include_str!("asm_snippets/push_constant.asm"), arg)
                    }

                    SegmentAddr::Static(arg) => {
                        format!(include_str!("asm_snippets/push_static.asm"), filename, arg)
                    }

                    SegmentAddr::Temp(arg) => {
                        format!(include_str!("asm_snippets/push_temp.asm"), arg)
                    }

                    SegmentAddr::Pointer(arg) => {
                        let addr = match arg {
                            0 => "THIS",
                            1 => "THAT",
                            _ => unreachable!(),
                        };

                        format!(include_str!("asm_snippets/push_pointer.asm"), addr)
                    }

                    ref segment @ (SegmentAddr::Local(arg)
                    | SegmentAddr::Arg(arg)
                    | SegmentAddr::This(arg)
                    | SegmentAddr::That(arg)) => {
                        let base_addr = match segment {
                            SegmentAddr::Local(_) => "LCL",
                            SegmentAddr::Arg(_) => "ARG",
                            SegmentAddr::This(_) => "THIS",
                            SegmentAddr::That(_) => "THAT",
                            _ => unreachable!(),
                        };

                        format!(
                            include_str!("asm_snippets/push_local_arg_this_that.asm"),
                            base_addr, arg
                        )
                    }
                },

                Inst::Pop(pop) => match pop {
                    SegmentAddr::Constant(_) => unreachable!(),

                    SegmentAddr::Static(arg) => {
                        format!(include_str!("asm_snippets/pop_static.asm"), filename, arg)
                    }

                    SegmentAddr::Temp(arg) => {
                        format!(include_str!("asm_snippets/pop_temp.asm"), arg)
                    }

                    SegmentAddr::Pointer(arg) => {
                        let addr = match arg {
                            0 => "THIS",
                            1 => "THAT",
                            _ => unreachable!(),
                        };

                        format!(include_str!("asm_snippets/pop_pointer.asm"), addr)
                    }

                    ref segment @ (SegmentAddr::Local(arg)
                    | SegmentAddr::Arg(arg)
                    | SegmentAddr::This(arg)
                    | SegmentAddr::That(arg)) => {
                        let base_addr = match segment {
                            SegmentAddr::Local(_) => "LCL",
                            SegmentAddr::Arg(_) => "ARG",
                            SegmentAddr::This(_) => "THIS",
                            SegmentAddr::That(_) => "THAT",
                            _ => unreachable!(),
                        };

                        format!(
                            include_str!("asm_snippets/pop_local_arg_this_that.asm"),
                            base_addr, arg
                        )
                    }
                },

                Inst::Add => format!(include_str!("asm_snippets/add.asm")),
                Inst::Sub => format!(include_str!("asm_snippets/sub.asm")),
                Inst::Neg => format!(include_str!("asm_snippets/neg.asm")),
                Inst::Eq => format!(include_str!("asm_snippets/eq.asm"), i),
                Inst::Gt => format!(include_str!("asm_snippets/gt.asm"), i),
                Inst::Lt => format!(include_str!("asm_snippets/lt.asm"), i),
                Inst::And => format!(include_str!("asm_snippets/and.asm")),
                Inst::Or => format!(include_str!("asm_snippets/or.asm")),
                Inst::Not => format!(include_str!("asm_snippets/not.asm")),

                Inst::Goto(label) => format!(include_str!("asm_snippets/goto.asm"), label),
                Inst::IfGoto(label) => format!(include_str!("asm_snippets/if_goto.asm"), label),
                Inst::Label(name) => format!(include_str!("asm_snippets/label.asm"), name),
            }
            .as_str(),
        )
    }

    asm
}

fn main() -> ExitCode {
    let Some(vm_file) = env::args().nth(1) else {
        eprintln!("A VM file path was not provided.");
        return ExitCode::FAILURE;
    };

    let vm_file = Path::new(vm_file.as_str());
    let mut parser = Parser::new(vm_file);
    if let Err(err_msg) = parser.parse() {
        eprintln!("{err_msg}");
        return ExitCode::FAILURE;
    }

    let vm_code = generate_vm_code(parser);
    fs::write(vm_file.with_extension("asm"), vm_code).unwrap();

    ExitCode::SUCCESS
}
