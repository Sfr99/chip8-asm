mod codegen;
mod lexer;
mod parser;

use codegen::generate;
use lexer::tokenize;
use parser::parse;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: chip8-asm <input.asm> <output.ch8>");
        return;
    }

    let source = fs::read_to_string(&args[1]).unwrap();
    let tokens = tokenize(&source);
    let (instructions, labels) = parse(tokens);
    let bytes = generate(instructions, labels);
    fs::write(&args[2], &bytes).unwrap();
}
