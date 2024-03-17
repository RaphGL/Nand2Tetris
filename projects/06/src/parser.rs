use std::fs;

#[derive(Debug)]
pub enum AddressInst {
    Value(u16),
    Symbol(String),
}

#[derive(Debug)]
pub enum CDest {
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

#[derive(Debug)]
pub enum CJump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

#[derive(Debug)]
pub enum CComp {
    Zero,
    One,
    NegOne,
    D,
    A,
    NotD,
    NotA,
    NegD,
    NegA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    NegM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug)]
pub struct ComputationInst {
    pub dest: CDest,
    pub comp: CComp,
    pub jump: CJump,
}

#[derive(Debug)]
pub enum Token {
    A(AddressInst),
    C(ComputationInst),
    Label(String),
}

fn parse_label(row: &str, line: i32) -> Result<Token, String> {
    let Some(mut row) = row.strip_prefix('(') else {
        return Err(format!("Invalid label syntax on line {}: {}", line, row));
    };

    row = match row.strip_suffix(')') {
        Some(row) => row,
        None => return Err(format!("Invalid label syntax on line {}: {}", line, row)),
    };

    Ok(Token::Label(row.into()))
}

fn parse_address_inst(row: &str, _line: i32) -> Result<Token, String> {
    let addr = &row[1..];
    Ok(match addr.parse::<u16>() {
        Ok(num) => Token::A(AddressInst::Value(num)),
        Err(_) => Token::A(AddressInst::Symbol(addr.into())),
    })
}

fn parse_computation_inst(row: &str, line: i32) -> Result<Token, String> {
    let mut row = row;
    let mut c_inst = ComputationInst {
        dest: CDest::Null,
        comp: CComp::Zero,
        jump: CJump::Null,
    };

    c_inst.dest = if let Some((dest, rest)) = row.split_once('=') {
        row = rest;
        match dest.trim() {
            "M" => CDest::M,
            "D" => CDest::D,
            "MD" => CDest::MD,
            "A" => CDest::A,
            "AM" => CDest::AM,
            "AD" => CDest::AD,
            "AMD" => CDest::AMD,
            _ => {
                return Err(format!(
                    "Found invalid destination on line {}: {}",
                    line, dest
                ))
            }
        }
    } else {
        CDest::Null
    };

    c_inst.jump = if let Some((rest, jump)) = row.split_once(';') {
        row = rest;
        match jump.trim() {
            "JGT" => CJump::JGT,
            "JEQ" => CJump::JEQ,
            "JGE" => CJump::JGE,
            "JLT" => CJump::JLT,
            "JNE" => CJump::JNE,
            "JLE" => CJump::JLE,
            "JMP" => CJump::JMP,
            _ => {
                return Err(format!(
                    "Found invalid jump condition on line {}: {}",
                    line, jump
                ))
            }
        }
    } else {
        CJump::Null
    };

    let mut comp = row.to_string();
    comp.retain(|c| !c.is_whitespace());
    c_inst.comp = match comp.as_str() {
        "0" => CComp::Zero,
        "1" => CComp::One,
        "-1" => CComp::NegOne,
        "D" => CComp::D,
        "A" => CComp::A,
        "!D" => CComp::NotD,
        "!A" => CComp::NotA,
        "-D" => CComp::NegD,
        "-A" => CComp::NegA,
        "D+1" => CComp::DPlusOne,
        "A+1" => CComp::APlusOne,
        "D-1" => CComp::DMinusOne,
        "A-1" => CComp::AMinusOne,
        "D+A" => CComp::DPlusA,
        "D-A" => CComp::DMinusA,
        "A-D" => CComp::AMinusD,
        "D&A" => CComp::DAndA,
        "D|A" => CComp::DOrA,
        "M" => CComp::M,
        "!M" => CComp::NotM,
        "-M" => CComp::NegM,
        "M+1" => CComp::MPlusOne,
        "M-1" => CComp::MMinusOne,
        "D+M" => CComp::DPlusM,
        "D-M" => CComp::DMinusM,
        "M-D" => CComp::MMinusD,
        "D&M" => CComp::DAndM,
        "D|M" => CComp::DOrM,
        _ => {
            return Err(format!(
                "Found invalid computation on line {}: {}",
                line, comp
            ))
        }
    };

    Ok(Token::C(c_inst))
}

pub fn parse_assembly(asm_file: &str) -> Vec<Token> {
    let asm = fs::read_to_string(asm_file).unwrap();
    let mut tokens = Vec::new();

    let mut line = 0;
    for row in asm.lines() {
        line += 1;
        let row = row.trim();
        if row.is_empty() || row.starts_with("//") {
            continue;
        }

        let row = if row.starts_with('@') {
            parse_address_inst(row, line)
        } else if row.starts_with('(') {
            parse_label(row, line)
        } else {
            parse_computation_inst(row, line)
        };

        tokens.push(match row {
            Ok(token) => token,
            Err(msg) => {
                eprintln!("{msg}");
                std::process::exit(1);
            }
        })
    }

    tokens
}
