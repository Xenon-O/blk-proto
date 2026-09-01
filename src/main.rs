use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;

use lexer::tokenize;
use parser::parse_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "examples/factorial.blk" };
    let src = fs::read_to_string(path).expect("Unable to read file");
    println!("blk-proto v0.1 — parsing '{}'\n", path);

    let tokens = tokenize(&src);
    println!("Tokens:\n{:?}\n", tokens);

    match parse_program(&tokens) {
        Ok(prog) => {
            println!("AST:\n{:#?}\n", prog);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
