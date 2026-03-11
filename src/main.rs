mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;

fn main() {
    let tokens = tokenize("CLS\nLD V3, #42");
    let instructions = parse(tokens);
    println!("{:?}", instructions);
}
