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
