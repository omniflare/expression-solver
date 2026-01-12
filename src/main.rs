use crate::lexer::Lexer;

mod input;
mod lexer;

fn main () {

    let blob = input::import_from_path("new.txt").unwrap();

    let mut lexer = Lexer::new(&blob);

    match lexer.tokenize() {
        Ok(tokens) => {
            for t in tokens {
                println!("{:?} ", t);
            }
        }
        Err(e) => {
            dbg!("found some issue");
        }
    }
}