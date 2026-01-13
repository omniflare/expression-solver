use crate::{lexer::Lexer, parser::Parser};

mod compiler;
mod input;
mod lexer;
mod parser;
mod vm;
mod utils;

fn main() {
    let blob = input::import_from_path("new.txt").unwrap();

    let mut lexer = Lexer::new(&blob);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error {}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return;
        }
    };

    println!("{:#?}", ast);
}
