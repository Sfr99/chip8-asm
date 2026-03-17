mod codegen;
mod lexer;
mod parser;

use codegen::generate;
use lexer::tokenize;
use parser::parse;
fn main() {
    let tokens = tokenize("loop: LD V0, #01\nJP loop");
    let (instructions, labels) = parse(tokens);
    let bytes = generate(instructions, labels);
    println!("{:02X?}", bytes);
}
