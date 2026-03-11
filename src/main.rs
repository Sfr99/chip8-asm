mod lexer;

use lexer::tokenize;

fn main() {
    let tokens = tokenize("loop: LD V3, #42");
    println!("{:?}", tokens);
}
