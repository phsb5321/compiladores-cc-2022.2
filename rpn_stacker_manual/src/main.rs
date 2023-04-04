use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num,
    Minus,
    Plus,
    Slash,
    Star,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

fn scan(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some('0'..='9') = chars.peek() {
                    num_str.push(chars.next().unwrap());
                }
                tokens.push(Token {
                    token_type: TokenType::Num,
                    lexeme: num_str,
                });
            }
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    lexeme: chars.next().unwrap().to_string(),
                });
            }
            '-' => {
                tokens.push(Token {
                    token_type: TokenType::Minus,
                    lexeme: chars.next().unwrap().to_string(),
                });
            }
            '*' => {
                tokens.push(Token {
                    token_type: TokenType::Star,
                    lexeme: chars.next().unwrap().to_string(),
                });
            }
            '/' => {
                tokens.push(Token {
                    token_type: TokenType::Slash,
                    lexeme: chars.next().unwrap().to_string(),
                });
            }
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            _ => {
                return Err(format!("Unexpected character: {}", c));
            }
        }
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: "".to_string(),
    });

    Ok(tokens)
}

fn process_file(file_path: &Path) {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    match scan(&content) {
        Ok(tokens) => {
            println!("Tokens:");
            for token in tokens {
                println!("{:?}", token);
            }
            // Evaluate the RPN expression and print the result
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: cargo run -- -f input.stk");
    }

    let file_path = Path::new(&args[2]);

    if file_path.is_file() {
        process_file(file_path);
    } else {
        panic!("Invalid file path");
    }
}
