use crate::parser::{AddressInst, CComp, CDest, CJump, ComputationInst, Token};
use std::collections::HashMap;

const VAR_START: u16 = 16;

pub struct Assembler {
    symbols: HashMap<String, u16>,
    tokens: Vec<Token>,
}

impl Assembler {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            symbols: HashMap::from([
                ("R0".into(), 0),
                ("R1".into(), 1),
                ("R2".into(), 2),
                ("R3".into(), 3),
                ("R4".into(), 4),
                ("R5".into(), 5),
                ("R6".into(), 6),
                ("R7".into(), 7),
                ("R8".into(), 8),
                ("R9".into(), 9),
                ("R10".into(), 10),
                ("R11".into(), 01),
                ("R12".into(), 12),
                ("R13".into(), 13),
                ("R14".into(), 14),
                ("R15".into(), 15),
                ("SCREEN".into(), 16384),
                ("KBD".into(), 24576),
                ("SP".into(), 0),
                ("LCL".into(), 1),
                ("ARG".into(), 2),
                ("THIS".into(), 3),
                ("THAT".into(), 4),
            ]),
        }
    }

    pub fn resolve_symbols(&mut self) {
        let mut line_count = 0;
        for token in &self.tokens {
            if let Token::Label(label) = token {
                if !self.symbols.contains_key(label) {
                    self.symbols.insert(label.clone(), line_count);
                }
            } else {
                line_count += 1;
            }
        }

        let mut var_count = VAR_START;
        for token in &self.tokens {
            if let Token::A(AddressInst::Symbol(addr)) = token {
                if !self.symbols.contains_key(addr) {
                    self.symbols.insert(addr.clone(), var_count);
                    var_count += 1;
                }
            }
        }
    }

    fn compile_a_instruction(&self, inst: &AddressInst) -> u16 {
        let addr = match inst {
            AddressInst::Value(val) => *val,
            AddressInst::Symbol(symbol) => *self.symbols.get(symbol).unwrap(),
        };

        0b0111111111111111 & addr
    }

    fn compile_c_instruction(&self, inst: &ComputationInst) -> u16 {
        let dest: u16 = match inst.dest {
            CDest::Null => 0,
            CDest::M => 0b001,
            CDest::D => 0b010,
            CDest::MD => 0b011,
            CDest::A => 0b100,
            CDest::AM => 0b101,
            CDest::AD => 0b110,
            CDest::AMD => 0b111,
        };

        let jump: u16 = match inst.jump {
            CJump::Null => 0,
            CJump::JGT => 0b001,
            CJump::JEQ => 0b010,
            CJump::JGE => 0b011,
            CJump::JLT => 0b100,
            CJump::JNE => 0b101,
            CJump::JLE => 0b110,
            CJump::JMP => 0b111,
        };

        let comp: u16 = match inst.comp {
            // a = 0
            CComp::Zero => 0b0101010,
            CComp::One => 0b0111111,
            CComp::NegOne => 0b0111010,
            CComp::D => 0b0001100,
            CComp::A => 0b0110000,
            CComp::NotD => 0b0001101,
            CComp::NotA => 0b0110001,
            CComp::NegD => 0b0001111,
            CComp::NegA => 0b0110011,
            CComp::DPlusOne => 0b0011111,
            CComp::APlusOne => 0b0110111,
            CComp::DMinusOne => 0b0001110,
            CComp::AMinusOne => 0b0110010,
            CComp::DPlusA => 0b0000010,
            CComp::DMinusA => 0b0010011,
            CComp::AMinusD => 0b0000111,
            CComp::DAndA => 0,
            CComp::DOrA => 0b0010101,
            // a = 1
            CComp::M => 0b1110000,
            CComp::NotM => 0b1110001,
            CComp::NegM => 0b1110011,
            CComp::MPlusOne => 0b1110111,
            CComp::MMinusOne => 0b1110010,
            CComp::DPlusM => 0b1000010,
            CComp::DMinusM => 0b1010011,
            CComp::MMinusD => 0b1000111,
            CComp::DAndM => 0b1000000,
            CComp::DOrM => 0b1010101,
        };

        0b1110000000000000 | (comp << 6) | (dest << 3) | jump
    }

    pub fn assemble(&self) -> Vec<u16> {
        let mut insts = Vec::new();
        for token in &self.tokens {
            let inst = match token {
                Token::A(a_inst) => self.compile_a_instruction(a_inst),
                Token::C(c_inst) => self.compile_c_instruction(c_inst),
                _ => continue,
            };

            insts.push(inst);
        }

        insts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_instruction_compilation() {
        let asm = Assembler::new(Vec::new());
        let inst = asm.compile_c_instruction(&ComputationInst {
            dest: CDest::MD,
            comp: CComp::DPlusOne,
            jump: CJump::JGT,
        });

        assert_eq!(inst, 0b1110011111011001);
    }

    #[test]
    fn a_instruction_compilation() {
        let asm = Assembler::new(Vec::new());
        let inst1 = asm.compile_a_instruction(&AddressInst::Symbol("R1".into()));
        assert_eq!(inst1, 0b0000000000000001);

        let inst2 = asm.compile_a_instruction(&AddressInst::Value(0xFFFF));
        assert_eq!(inst2, 0xFFFF >> 1);
    }
}
