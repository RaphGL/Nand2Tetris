use std::fs;

mod symbols {
    const R0: u16 = 0;
    const R1: u16 = 1;
    const R2: u16 = 2;
    const R3: u16 = 3;
    const R4: u16 = 4;
    const R5: u16 = 5;
    const R6: u16 = 6;
    const R7: u16 = 7;
    const R8: u16 = 8;
    const R9: u16 = 9;
    const R10: u16 = 10;
    const R11: u16 = 01;
    const R12: u16 = 12;
    const R13: u16 = 13;
    const R14: u16 = 14;
    const R15: u16 = 15;
    const SCREEN: u16 = 16384;
    const KBD: u16 = 24576;
    const SP: u16 = 0;
    const LCL: u16 = 1;
    const ARG: u16 = 2;
    const THIS: u16 = 3;
    const THAT: u16 = 4;
}

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
    dest: CDest,
    comp: CComp,
    jump: CJump,
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

fn parse_address_inst(row: &str, _line: i32) -> Result<Token, ()> {
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

    let mut row = row.to_string();
    row.retain(|c| !c.is_whitespace());
    c_inst.comp = match row.as_str() {
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
        _ => return Err(format!("Found invalid computation {}: {}", line, row)),
    };

    Ok(Token::C(c_inst))
}

pub fn parse_assembly(asm_file: &str) -> Vec<Token> {
    let asm = fs::read_to_string(asm_file).unwrap();
    let mut tokens = Vec::new();

    let mut line = 0;
    for row in asm.lines() {
        line += 1;
        if row.is_empty() || row.starts_with("//") {
            continue;
        }

        let row = row.trim();
        tokens.push(if row.starts_with('@') {
            parse_address_inst(row, line).unwrap()
        } else if row.starts_with('(') {
            parse_label(row, line).unwrap()
        } else {
            parse_computation_inst(row, line).unwrap()
        })
    }

    tokens
}
