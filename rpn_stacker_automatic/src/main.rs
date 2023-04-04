use regex::Regex;
use std::env;
use std::fs;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

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
    token_type: TokenType,
    lexeme: String,
}

struct RegexScanner {
    regex: Regex,
}

impl RegexScanner {
    pub fn new() -> Self {
        let regex = Regex::new(r"(\d+|[+\-*/])").unwrap();
        Self { regex }
    }

    pub fn scan(&self, source: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        for capture in self.regex.captures_iter(source) {
            let lexeme = capture.get(0).unwrap().as_str().to_string();

            let token_type = match lexeme.chars().next().unwrap() {
                '0'..='9' => TokenType::Num,
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Star,
                '/' => TokenType::Slash,
                _ => return Err(format!("Unexpected character: {}", lexeme)),
            };

            tokens.push(Token { token_type, lexeme });
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
        });

        Ok(tokens)
    }
}

fn process_file(file_path: &Path) {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let scanner = RegexScanner::new();
    let tokens = scanner.scan(&content).expect("Failed to scan the source");

    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token);
    }

    // Evaluate the RPN expression and print the result
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
