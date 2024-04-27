use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

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
    Function(String, u16),
    Call(String, u16),
    Return,
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

            let line_parts: Vec<_> = line.split_whitespace().map(|part| part.trim()).collect();

            self.tokens.push(match line_parts[0] {
                "push" => {
                    let arg = line_parts[2].parse::<_>().unwrap();
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

                "function" => {
                    Inst::Function(line_parts[1].into(), line_parts[2].parse::<_>().unwrap())
                }
                "call" => Inst::Call(line_parts[1].into(), line_parts[2].parse::<_>().unwrap()),
                "return" => Inst::Return,

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
    let file = Path::new(parser.file);
    let filename = file.file_stem().unwrap().to_str().unwrap();
    let mut ret_no = 0;

    let mut asm = String::from({
        let call_sys_init = format!(
            include_str!("asm_snippets/call.asm",),
            "Sys.init", 0, ret_no
        );

        format!(include_str!("asm_snippets/init.asm"), call_sys_init)
    });

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

                Inst::Function(name, vars_no) => {
                    format!(include_str!("asm_snippets/function.asm"), name, vars_no)
                }
                Inst::Return => format!(include_str!("asm_snippets/return.asm")),
                Inst::Call(name, args_no) => {
                    ret_no += 1;
                    let call = format!(
                        include_str!("asm_snippets/call.asm",),
                        name, args_no, ret_no
                    );
                    call
                }
            }
            .as_str(),
        )
    }

    asm
}

fn main() -> ExitCode {
    let mut args = env::args();
    if args.len() < 2 {
        eprintln!("A VM file or directory was not provided.");
        return ExitCode::FAILURE;
    }

    let input = PathBuf::from(args.nth(1).unwrap());
    let mut output_file = String::new();

    let compile_vm_file = |vm_file: PathBuf| -> Result<String, String> {
        let mut parser = Parser::new(vm_file.as_path());
        if let Err(err_msg) = parser.parse() {
            return Err(err_msg);
        }

        Ok(generate_vm_code(parser))
    };

    if input.is_dir() {
        for file in fs::read_dir(input).unwrap() {
            let file = file.unwrap().path();
            let ext = file.extension().unwrap().to_str().unwrap();
            if ext == "vm" {
                output_file += compile_vm_file(file).unwrap().as_str();
            }
        }
    } else {
        output_file += compile_vm_file(input).unwrap().as_str();
    }

    let filename = {
        let curr_dir = env::current_dir().unwrap();
        let mut mut_curr_dir = curr_dir.clone();
        mut_curr_dir.push(curr_dir.file_name().unwrap().to_str().unwrap());
        mut_curr_dir
    };
    fs::write(filename.with_extension("asm"), output_file).unwrap();

    ExitCode::SUCCESS
}
