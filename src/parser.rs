/* chip8-asm -- MIT License -- Javier Salafranca Pradilla -- 2026*/
use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
    Label(String),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

pub fn parse(tokens: Vec<Token>) -> (Vec<Instruction>, HashMap<String, usize>) {
    let mut i = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::Identifier(name) => {
                if i + 1 < tokens.len() && tokens[i + 1] == Token::Colon {
                    labels.insert(name.clone(), instructions.len());
                    i += 2;
                    continue;
                }
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
                                Operand::Label(op_name.clone())
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
            Token::Number(_) => {}
            Token::Comma => {}
            Token::Colon => {}
            Token::NewLine => {}
            Token::LBracket => {}
            Token::RBracket => {}
        }
        i += 1;
    }
    (instructions, labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_no_operands() {
        let tokens = tokenize("CLS\n");
        let (instructions, _) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::CLS,
                operands: vec![],
            }]
        );
    }

    #[test]
    fn test_one_operand() {
        let tokens = tokenize("JP #200\n");
        let (instructions, _) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::JP,
                operands: vec![Operand::Immediate(0x200)],
            }]
        );
    }

    #[test]
    fn test_two_operands() {
        let tokens = tokenize("LD V3, #42\n");
        let (instructions, _) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::LD,
                operands: vec![Operand::Register(3), Operand::Immediate(0x42)],
            }]
        );
    }

    #[test]
    fn test_three_operands() {
        let tokens = tokenize("DRW V0, V1, #5\n");
        let (instructions, _) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::DRW,
                operands: vec![
                    Operand::Register(0),
                    Operand::Register(1),
                    Operand::Immediate(0x5)
                ],
            }]
        );
    }

    #[test]
    fn test_special_operands() {
        let tokens = tokenize("LD DT, V0\n");
        let (instructions, _) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::LD,
                operands: vec![Operand::DT, Operand::Register(0)],
            }]
        );
    }

    #[test]
    fn test_ideref() {
        let tokens = tokenize("LD [I], V5\n");
        let (instructions, _) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::LD,
                operands: vec![Operand::IDeref, Operand::Register(5)],
            }]
        );
    }

    #[test]
    fn test_label_definition() {
        let tokens = tokenize("loop:\nCLS\n");
        let (instructions, labels) = parse(tokens);
        assert_eq!(
            instructions,
            vec![Instruction {
                mnemonic: Mnemonic::CLS,
                operands: vec![],
            }]
        );
        assert_eq!(labels["loop"], 0);
    }

    #[test]
    fn test_label_reference() {
        let tokens = tokenize("start:\nLD V0, #1\nJP start\n");
        let (instructions, labels) = parse(tokens);
        assert_eq!(
            instructions[1],
            Instruction {
                mnemonic: Mnemonic::JP,
                operands: vec![Operand::Label("start".to_string())],
            }
        );
        assert_eq!(labels["start"], 0);
    }
}
