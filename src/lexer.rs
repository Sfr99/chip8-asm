/* chip8-asm -- MIT License -- Javier Salafranca Pradilla -- 2026*/
#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(u16),
    Colon,
    Comma,
    NewLine,
    LBracket,
    RBracket,
}

pub fn tokenize(entry: &str) -> Vec<Token> {
    let chars: Vec<char> = entry.chars().collect();
    let mut i = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while i < chars.len() {
        match chars[i] {
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            '\n' => {
                tokens.push(Token::NewLine);
                i += 1;
            }
            ';' => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            ' ' => {
                i += 1;
            }
            '[' => {
                tokens.push(Token::LBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RBracket);
                i += 1;
            }
            _ => {
                let start = i;
                if chars[i] == '#' {
                    i += 1;
                    while i < chars.len() && chars[i].is_ascii_hexdigit() {
                        i += 1;
                    }
                    let hex_str: String = chars[start + 1..i].iter().collect();
                    let number = u16::from_str_radix(&hex_str, 16).unwrap();
                    tokens.push(Token::Number(number));
                } else {
                    while i < chars.len() && chars[i].is_alphanumeric() {
                        i += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    tokens.push(Token::Identifier(word));
                }
            }
        }
    }
    tokens
}
