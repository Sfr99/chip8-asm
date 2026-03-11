mod lexer;

use lexer::lexer;

fn main() {
    let tokens = lexer("loop: LD V3, #42");
    println!("{:?}", tokens);
}
