use crate::lexer::tokenize;
use crate::parser::parse_program;

fn main() {
    let src = std::fs::read_to_string("examples/Math.blk").expect("read");
    let toks = tokenize(&src);
    println!("TOKENS: {:?}\n", toks);
    let prog = parse_program(&toks).unwrap();
    println!("AST: {:#?}\n", prog);
}
