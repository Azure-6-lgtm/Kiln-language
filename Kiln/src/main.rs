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
mod interpreter;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Kiln Programming Language V0.0.1 alpha");
    println!("Made by Azure-6-lgtm");
    println!("-----------------------------");

	if args.len() < 2 {
		println!("Usage: Kiln [OPTION] [FILE]");
		return;
	}

	match args[1].as_str() {
		"--ver" => {
			println!("Kiln Programming Language V0.0.1 alpha");
			return;
		},
		_ => {},
	};

	let filename = &args[1];

	if !filename.ends_with(".kiln") {
		eprintln!("Kiln: File is not a valid kiln file, A valid Kiln file must end with .kiln");
		return;
	}

    let source = match fs::read_to_string(filename) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Kiln: cannot read '{}': {}", filename, error);
            process::exit(1);
        }
    };

    
    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!(
                "Kiln: lexer error at {}:{}: {}",
                error.line, error.column, error.message
            );
            process::exit(1);
        }
    };

    
    let mut parser = Parser::new(tokens);

    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "Kiln: parser error at {}:{}: {}",
                error.line, error.column, error.message
            );
            process::exit(1);
        }
    };
    
    let mut interpreter = Interpreter::new(&program);

    if let Err(error) = interpreter.run() {
        eprintln!("Kiln: runtime error: {}", error);
        process::exit(1);
    }
}
