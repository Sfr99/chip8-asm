/* chip8-asm -- MIT License -- Javier Salafranca Pradilla -- 2026*/
use crate::parser::Instruction;
use crate::parser::Mnemonic;
use crate::parser::Operand;
use std::collections::HashMap;

fn emit(bytes: &mut Vec<u8>, opcode: u16) {
    bytes.push((opcode >> 8) as u8);
    bytes.push((opcode & 0xFF) as u8);
}

pub fn generate(instructions: Vec<Instruction>, labels: HashMap<String, usize>) -> Vec<u8> {
    let mut i = 0;
    let mut bytes: Vec<u8> = Vec::new();

    while i < instructions.len() {
        match &instructions[i].mnemonic {
            Mnemonic::CLS => {
                emit(&mut bytes, 0x00E0);
            }
            Mnemonic::RET => {
                emit(&mut bytes, 0x00EE);
            }
            Mnemonic::SYS => {
                //deprecated
                panic!("SYS not supported");
            }
            Mnemonic::JP => {
                let ops = &instructions[i].operands;
                match &ops[0] {
                    Operand::Immediate(addr) => {
                        emit(&mut bytes, 0x1000 | addr);
                    }
                    Operand::Label(label) => {
                        emit(&mut bytes, 0x1000 | ((0x200 + (labels[label] * 2)) as u16));
                    }
                    _ => panic!("Invalid operand for JP"),
                }
            }
            Mnemonic::CALL => {
                let ops = &instructions[i].operands;
                match &ops[0] {
                    Operand::Immediate(addr) => {
                        emit(&mut bytes, 0x2000 | addr);
                    }
                    Operand::Label(label) => {
                        emit(&mut bytes, 0x2000 | ((0x200 + (labels[label] * 2)) as u16));
                    }
                    _ => panic!("Invalid operand for CALL"),
                }
            }
            Mnemonic::SE => {
                let ops = &instructions[i].operands;

                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("SE expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => emit(&mut bytes, 0x5000 | (x << 8) | ((y as u16) << 4)),
                    Operand::Immediate(kk) => emit(&mut bytes, 0x3000 | (x << 8) | (kk as u16)),
                    _ => panic!("Invalid operand for SE"),
                };
            }
            Mnemonic::SNE => {
                let ops = &instructions[i].operands;

                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("SNE expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => emit(&mut bytes, 0x9000 | (x << 8) | ((y as u16) << 4)),
                    Operand::Immediate(kk) => emit(&mut bytes, 0x4000 | (x << 8) | (kk as u16)),
                    _ => panic!("Invalid operand for SNE"),
                };
            }
            Mnemonic::ADD => {
                let ops = &instructions[i].operands;
                match ops[0] {
                    Operand::Register(x) => {
                        match ops[1] {
                            Operand::Register(y) => emit(
                                &mut bytes,
                                0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 4,
                            ),
                            Operand::Immediate(kk) => {
                                emit(&mut bytes, 0x7000 | ((x as u16) << 8) | (kk as u16))
                            }
                            _ => panic!("Invalid operand for ADD"),
                        };
                    }
                    Operand::IRegister => match ops[1] {
                        Operand::Register(x) => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x1E),
                        _ => panic!("ADD I expects register"),
                    },
                    _ => panic!("Invalid operand for ADD"),
                };
            }
            Mnemonic::LD => {
                let ops = &instructions[i].operands;
                match ops[0] {
                    Operand::Register(x) => {
                        match ops[1] {
                            Operand::Immediate(nn) => {
                                emit(&mut bytes, 0x6000 | ((x as u16) << 8) | (nn as u16))
                            }
                            Operand::Register(y) => {
                                emit(&mut bytes, 0x8000 | ((x as u16) << 8) | ((y as u16) << 4))
                            }
                            Operand::DT => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x07),
                            Operand::Key => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x0A),
                            Operand::IDeref => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x65),
                            _ => panic!("Invalid operand for LD"),
                        };
                    }
                    Operand::IRegister => match ops[1] {
                        Operand::Immediate(nnn) => emit(&mut bytes, 0xA000 | (nnn as u16)),
                        _ => panic!("LD expects address"),
                    },
                    Operand::DT => match ops[1] {
                        Operand::Register(x) => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x15),
                        _ => panic!("LD DT expects register"),
                    },
                    Operand::ST => match ops[1] {
                        Operand::Register(x) => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x18),
                        _ => panic!("LD ST expects register"),
                    },
                    Operand::F => match ops[1] {
                        Operand::Register(x) => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x29),
                        _ => panic!("LD F expects register"),
                    },
                    Operand::B => match ops[1] {
                        Operand::Register(x) => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x33),
                        _ => panic!("LD B expects register"),
                    },
                    Operand::IDeref => match ops[1] {
                        Operand::Register(x) => emit(&mut bytes, 0xF000 | ((x as u16) << 8) | 0x55),
                        _ => panic!("LD [I] expects register"),
                    },
                    _ => panic!("Invalid operand for LD"),
                };
            }
            Mnemonic::OR => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("OR expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 1)
                    }
                    _ => panic!("Invalid operand for OR"),
                };
            }
            Mnemonic::AND => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("AND expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 2)
                    }
                    _ => panic!("Invalid operand for AND"),
                };
            }
            Mnemonic::XOR => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("XOR expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 3)
                    }
                    _ => panic!("Invalid operand for XOR"),
                };
            }
            Mnemonic::SUB => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("SUB expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 5)
                    }
                    _ => panic!("Invalid operand for SUB"),
                };
            }
            Mnemonic::SHR => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("SHR expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 6)
                    }
                    _ => panic!("Invalid operand for SHR"),
                };
            }
            Mnemonic::SUBN => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("SUBN expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 7)
                    }
                    _ => panic!("Invalid operand for SUBN"),
                };
            }
            Mnemonic::SHL => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("SHL expects register"),
                };
                match ops[1] {
                    Operand::Register(y) => {
                        emit(&mut bytes, 0x8000 | (x << 8) | ((y as u16) << 4) | 0xE)
                    }
                    _ => panic!("Invalid operand for SHL"),
                };
            }
            Mnemonic::RND => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("RND expects register"),
                };
                match ops[1] {
                    Operand::Immediate(kk) => emit(&mut bytes, 0xC000 | (x << 8) | (kk as u16)),
                    _ => panic!("Invalid operand for RND"),
                };
            }
            Mnemonic::DRW => {
                let ops = &instructions[i].operands;
                let x = match ops[0] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("DRW expects register"),
                };
                let y = match ops[1] {
                    Operand::Register(r) => r as u16,
                    _ => panic!("DRW expects register"),
                };
                match ops[2] {
                    Operand::Immediate(n) => {
                        emit(&mut bytes, 0xD000 | (x << 8) | (y << 4) | (n as u16))
                    }

                    _ => panic!("Invalid operand for DRW"),
                }
            }
            Mnemonic::SKP => {
                let ops = &instructions[i].operands;
                match ops[0] {
                    Operand::Register(x) => emit(&mut bytes, 0xE000 | ((x as u16) << 8) | 0x9E),
                    _ => panic!("SKP expects register"),
                };
            }
            Mnemonic::SKNP => {
                let ops = &instructions[i].operands;
                match ops[0] {
                    Operand::Register(x) => emit(&mut bytes, 0xE000 | ((x as u16) << 8) | 0xA1),
                    _ => panic!("SKNP expects register"),
                };
            }
        }
        i += 1;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn assemble(source: &str) -> Vec<u8> {
        let tokens = tokenize(source);
        let (instructions, labels) = parse(tokens);
        generate(instructions, labels)
    }

    #[test]
    fn test_cls() {
        assert_eq!(assemble("CLS\n"), vec![0x00, 0xE0]);
    }

    #[test]
    fn test_ret() {
        assert_eq!(assemble("RET\n"), vec![0x00, 0xEE]);
    }

    #[test]
    fn test_jp() {
        assert_eq!(assemble("JP #200\n"), vec![0x12, 0x00]);
    }

    #[test]
    fn test_call() {
        assert_eq!(assemble("CALL #400\n"), vec![0x24, 0x00]);
    }

    #[test]
    fn test_se_immediate() {
        assert_eq!(assemble("SE V3, #42\n"), vec![0x33, 0x42]);
    }

    #[test]
    fn test_se_register() {
        assert_eq!(assemble("SE V3, V5\n"), vec![0x53, 0x50]);
    }

    #[test]
    fn test_sne_immediate() {
        assert_eq!(assemble("SNE VA, #FF\n"), vec![0x4A, 0xFF]);
    }

    #[test]
    fn test_ld_byte() {
        assert_eq!(assemble("LD V0, #FF\n"), vec![0x60, 0xFF]);
    }

    #[test]
    fn test_ld_register() {
        assert_eq!(assemble("LD V1, V2\n"), vec![0x81, 0x20]);
    }

    #[test]
    fn test_ld_i() {
        assert_eq!(assemble("LD I, #300\n"), vec![0xA3, 0x00]);
    }

    #[test]
    fn test_ld_dt_read() {
        assert_eq!(assemble("LD V5, DT\n"), vec![0xF5, 0x07]);
    }

    #[test]
    fn test_ld_dt_write() {
        assert_eq!(assemble("LD DT, V5\n"), vec![0xF5, 0x15]);
    }

    #[test]
    fn test_ld_st() {
        assert_eq!(assemble("LD ST, V3\n"), vec![0xF3, 0x18]);
    }

    #[test]
    fn test_ld_key() {
        assert_eq!(assemble("LD VA, K\n"), vec![0xFA, 0x0A]);
    }

    #[test]
    fn test_ld_font() {
        assert_eq!(assemble("LD F, V7\n"), vec![0xF7, 0x29]);
    }

    #[test]
    fn test_ld_bcd() {
        assert_eq!(assemble("LD B, V3\n"), vec![0xF3, 0x33]);
    }

    #[test]
    fn test_ld_store() {
        assert_eq!(assemble("LD [I], V5\n"), vec![0xF5, 0x55]);
    }

    #[test]
    fn test_ld_load() {
        assert_eq!(assemble("LD V5, [I]\n"), vec![0xF5, 0x65]);
    }

    #[test]
    fn test_add_byte() {
        assert_eq!(assemble("ADD V1, #10\n"), vec![0x71, 0x10]);
    }

    #[test]
    fn test_add_register() {
        assert_eq!(assemble("ADD V1, V2\n"), vec![0x81, 0x24]);
    }

    #[test]
    fn test_add_i() {
        assert_eq!(assemble("ADD I, V6\n"), vec![0xF6, 0x1E]);
    }

    #[test]
    fn test_or() {
        assert_eq!(assemble("OR V1, V2\n"), vec![0x81, 0x21]);
    }

    #[test]
    fn test_and() {
        assert_eq!(assemble("AND V1, V2\n"), vec![0x81, 0x22]);
    }

    #[test]
    fn test_xor() {
        assert_eq!(assemble("XOR V1, V2\n"), vec![0x81, 0x23]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(assemble("SUB V1, V2\n"), vec![0x81, 0x25]);
    }

    #[test]
    fn test_shr() {
        assert_eq!(assemble("SHR V1, V2\n"), vec![0x81, 0x26]);
    }

    #[test]
    fn test_subn() {
        assert_eq!(assemble("SUBN V1, V2\n"), vec![0x81, 0x27]);
    }

    #[test]
    fn test_shl() {
        assert_eq!(assemble("SHL V1, V2\n"), vec![0x81, 0x2E]);
    }

    #[test]
    fn test_rnd() {
        assert_eq!(assemble("RND V0, #FF\n"), vec![0xC0, 0xFF]);
    }

    #[test]
    fn test_drw() {
        assert_eq!(assemble("DRW V0, V1, #5\n"), vec![0xD0, 0x15]);
    }

    #[test]
    fn test_skp() {
        assert_eq!(assemble("SKP V3\n"), vec![0xE3, 0x9E]);
    }

    #[test]
    fn test_sknp() {
        assert_eq!(assemble("SKNP V3\n"), vec![0xE3, 0xA1]);
    }

    #[test]
    fn test_label_jp() {
        let bytes = assemble("loop:\nLD V0, #1\nJP loop\n");
        assert_eq!(bytes, vec![0x60, 0x01, 0x12, 0x00]);
    }

    #[test]
    fn test_forward_label() {
        let bytes = assemble("JP end\nCLS\nend:\nRET\n");
        assert_eq!(bytes, vec![0x12, 0x04, 0x00, 0xE0, 0x00, 0xEE]);
    }
}
