/*
        ,-~~~-.
      ,'  ) (  '.
     /   (   )   \
    |   .-----.   |
    |  /  ~ ~  \  |
  __|_|_________|_|__

 _    _ _
| | _(_) |_ __
| |/ / | | '_ \
|   <| | | | | |
|_|\_\_|_|_| |_|
   source in, solid out

  Kiln version 0.0.1 alpha pre-release
*/

mod ast;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("Kiln Programming Language V0.0.1 alpha pre-release");

    let source = match fs::read_to_string("main.kiln") {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Kiln: {}", error);
            return Ok(());
        }
    };

    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Kiln: {}:{}: {}", error.line, error.column, error.message);
            return Ok(());
        }
    };

    let mut parser = Parser::new(tokens.clone());

    println!("{tokens:#?}");

    match parser.parse_program() {
        Ok(program) => {
            println!("{program:#?}");
        }
        Err(error) => {
            eprintln!("Kiln: {}:{}: {}", error.line, error.column, error.message);
        }
    }

    Ok(())
}
