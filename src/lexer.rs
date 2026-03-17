/* chip8-asm -- MIT License -- Javier Salafranca Pradilla -- 2026*/
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let tokens = tokenize("LD V3, #42\n");
        let expected = vec![
            Token::Identifier("LD".to_string()),
            Token::Identifier("V3".to_string()),
            Token::Comma,
            Token::Number(0x42),
            Token::NewLine,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let tokens = tokenize("LD V3, #42 ; This is a comment\n");
        let expected = vec![
            Token::Identifier("LD".to_string()),
            Token::Identifier("V3".to_string()),
            Token::Comma,
            Token::Number(0x42),
            Token::NewLine,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_brackets() {
        let tokens = tokenize("LD V0, [I]\n");
        let expected = vec![
            Token::Identifier("LD".to_string()),
            Token::Identifier("V0".to_string()),
            Token::Comma,
            Token::LBracket,
            Token::Identifier("I".to_string()),
            Token::RBracket,
            Token::NewLine,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_labels() {
        let tokens = tokenize("start:\nLD V0, #0\n");
        let expected = vec![
            Token::Identifier("start".to_string()),
            Token::Colon,
            Token::NewLine,
            Token::Identifier("LD".to_string()),
            Token::Identifier("V0".to_string()),
            Token::Comma,
            Token::Number(0x0),
            Token::NewLine,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn complex_example() {
        let tokens = tokenize("loop:\n  LD V0, #FF\n  LD [I], V0\n  ; comment\n  JP loop\n");
        let expected = vec![
            Token::Identifier("loop".to_string()),
            Token::Colon,
            Token::NewLine,
            Token::Identifier("LD".to_string()),
            Token::Identifier("V0".to_string()),
            Token::Comma,
            Token::Number(0xFF),
            Token::NewLine,
            Token::Identifier("LD".to_string()),
            Token::LBracket,
            Token::Identifier("I".to_string()),
            Token::RBracket,
            Token::Comma,
            Token::Identifier("V0".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Identifier("JP".to_string()),
            Token::Identifier("loop".to_string()),
            Token::NewLine,
        ];
        assert_eq!(tokens, expected);
    }
}
