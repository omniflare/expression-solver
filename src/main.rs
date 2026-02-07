use std::env;
use std::fs::OpenOptions;

use crate::{compiler::compile, lexer::Lexer, parser::Parser, vm::run_program};

mod compiler;
mod input;
mod lexer;
mod parser;
mod utils;
mod vm;

fn main() {
    let path = env::args().nth(1).expect("Provide file path");

    let blob = input::import_from_path(&path).unwrap();

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

    let program = compile(&ast);

    println!("\nBYTECODE:");
    println!("{:?}", program);

    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("log.log")
        .expect("Failed to open log file");

    match run_program(program, &mut log_file) {
        Ok(Some(result)) => println!("\nRESULT = {}", result),
        Ok(None) => println!("Program finished with empty stack"),
        Err(_) => println!("Runtime error"),
    }
}
