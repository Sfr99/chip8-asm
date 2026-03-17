/* chip8-asm -- MIT License -- Javier Salafranca Pradilla -- 2026*/
use crate::lexer::Token;

#[derive(Debug)]
pub enum Operand {
    Register(u8),
    IRegister,
    DT,
    ST,
    Immediate(u16),
    Key,
    F,
    B,
    IDeref,
}

#[derive(Debug)]
pub enum Mnemonic {
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE,
    SNE,
    ADD,
    LD,
    OR,
    AND,
    XOR,
    SUB,
    SHR,
    SUBN,
    RND,
    DRW,
    SKP,
    SKNP,
    SHL,
}

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut i = 0;
    let mut instructions: Vec<Instruction> = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::Identifier(name) => {
                let mnemonic = match name.as_str() {
                    "CLS" => Mnemonic::CLS,
                    "RET" => Mnemonic::RET,
                    "SYS" => Mnemonic::SYS,
                    "JP" => Mnemonic::JP,
                    "CALL" => Mnemonic::CALL,
                    "SE" => Mnemonic::SE,
                    "SNE" => Mnemonic::SNE,
                    "ADD" => Mnemonic::ADD,
                    "LD" => Mnemonic::LD,
                    "OR" => Mnemonic::OR,
                    "AND" => Mnemonic::AND,
                    "XOR" => Mnemonic::XOR,
                    "SUB" => Mnemonic::SUB,
                    "SHR" => Mnemonic::SHR,
                    "SUBN" => Mnemonic::SUBN,
                    "SHL" => Mnemonic::SHL,
                    "RND" => Mnemonic::RND,
                    "DRW" => Mnemonic::DRW,
                    "SKP" => Mnemonic::SKP,
                    "SKNP" => Mnemonic::SKNP,
                    _ => panic!("Unknown mnemonic: {}", name),
                };
                i += 1;
                let mut operands: Vec<Operand> = Vec::new();
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Identifier(op_name) => {
                            let operand = if op_name.starts_with('V') {
                                let hex = &op_name[1..];
                                let reg = u8::from_str_radix(hex, 16).unwrap();
                                Operand::Register(reg)
                            } else if op_name == "I" {
                                Operand::IRegister
                            } else if op_name == "DT" {
                                Operand::DT
                            } else if op_name == "ST" {
                                Operand::ST
                            } else if op_name == "K" {
                                Operand::Key
                            } else if op_name == "F" {
                                Operand::F
                            } else if op_name == "B" {
                                Operand::B
                            } else {
                                panic!("Unknown operand: {}", op_name)
                            };
                            operands.push(operand);
                        }
                        Token::Number(num) => {
                            operands.push(Operand::Immediate(*num));
                        }
                        Token::Comma => {}
                        Token::NewLine => break,
                        Token::LBracket => {
                            i += 2;
                            operands.push(Operand::IDeref);
                        }
                        _ => break,
                    }
                    i += 1;
                }
                instructions.push(Instruction { mnemonic, operands });
            }
            Token::Number(num) => {}
            Token::Comma => {}
            Token::Colon => {}
            Token::NewLine => {}
            Token::LBracket => {}
            Token::RBracket => {}
        }
        i += 1;
    }
    instructions
}
