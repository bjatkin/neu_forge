use std::io::{self, Write};

mod ast;
mod interp;
mod parser;

fn main() {
    let mut input = String::new();
    let mut interpreter = interp::Interp::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "exit" {
            break;
        }
        let mut parser = parser::Parser::new(trimmed);
        let ast = parser.parse();
        let result = interpreter.interp(ast);
        println!("\t{result}")
    }
}
