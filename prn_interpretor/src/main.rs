use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

#[derive(Parser)]
#[clap(
    name = "RPN Stacker",
    version = "0.1.0",
    author = "Pedro Henrique Souza Balbino"
)]
struct Args {
    #[clap(long, short = 'f', required = true)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let file_path = Path::new(&args.input_file);

    if !file_path.is_file() || !is_valid_extension(&file_path) {
        eprintln!("Invalid file path or extension");
        return;
    }

    process_file(&file_path);
}

fn is_valid_extension(file_path: &Path) -> bool {
    file_path
        .extension()
        .map_or(false, |ext| ext == "txt" || ext == "stk")
}

fn process_file(file_path: &Path) {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", file_path.display(), e);
            return;
        }
    };

    let mut buf_reader = BufReader::new(file);
    let mut rpn_expression = String::new();

    if let Err(e) = buf_reader.read_to_string(&mut rpn_expression) {
        eprintln!("Failed to read file {}: {}", file_path.display(), e);
        return;
    }

    match evaluate_rpn(&rpn_expression.trim()) {
        Some(result) => println!("Result for {}: {}", file_path.display(), result),
        None => eprintln!("Invalid RPN expression in {}", file_path.display()),
    }
}

fn evaluate_rpn(expression: &str) -> Option<f64> {
    let mut stack = Vec::new();

    for token in expression.split_whitespace() {
        if let Ok(number) = f64::from_str(token) {
            stack.push(number);
        } else {
            let rhs = stack.pop()?;
            let lhs = stack.pop()?;
            let result = match token {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                _ => return None,
            };
            stack.push(result);
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
