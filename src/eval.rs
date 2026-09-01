use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

mod lexer;
mod parser;
mod ast;

use lexer::tokenize;
use parser::{parse_program, Program, Expr, Stmt};
use ast::Value;

#[derive(Default)]
struct Env {
    vals: HashMap<String, Value>,
    members: HashMap<String, Vec<String>>, // simple membership/type declarations
}

impl Env {
    fn new() -> Self {
        let mut e = Env::default();
        // Seed true/false as selector functions
        e.vals.insert("true".into(), Value::BoolSelector(true));
        e.vals.insert("false".into(), Value::BoolSelector(false));
        e
    }
}

fn eval_program(prog: &Program, env: &mut Env) -> Result<Option<Value>> {
    let mut last = None;
    for stmt in &prog.stmts {
        match stmt {
            Stmt::Expr(e) => { last = Some(eval_expr(e, env)?); }
        }
    }
    Ok(last)
}

fn eval_expr(expr: &Expr, env: &mut Env) -> Result<Value> {
    match expr {
        Expr::Number(n) => Ok(Value::Int(*n)),
        Expr::Ident(s) => Ok(env.vals.get(s).cloned().unwrap_or(Value::None)),
        Expr::String(s) => Ok(Value::Str(s.clone())),
        Expr::Tuple(items) => {
            let mut vals = Vec::new();
            for it in items { vals.push(eval_expr(it, env)?); }
            Ok(Value::Tuple(vals))
        }
        Expr::Assign(name, rhs) => {
            let val = eval_expr(rhs, env)?;
            if !env.vals.contains_key(name) {
                env.vals.insert(name.clone(), val.clone());
                Ok(val)
            } else {
                let existing = env.vals.get(name).cloned().unwrap_or(Value::None);
                Ok(Value::Bool(existing == val))
            }
        }
        Expr::MappingArgs(items) => {
            let mut vals = Vec::new();
            for it in items { vals.push(eval_expr(it, env)?); }
            Ok(Value::Tuple(vals))
        }
        Expr::MappingLiteral(entries) => {
            let mut map = Vec::new();
            for (k, v) in entries {
                let kv = eval_expr(k, env)?;
                let vv = eval_expr(v, env)?;
                map.push((kv, vv));
            }
            Ok(Value::Mapping(map))
        }
        Expr::Backtick(tag, content) => Ok(Value::Backtick(tag.clone(), content.clone())),
        Expr::Wildcard => Ok(Value::None),
    }
}

fn repl() -> Result<()> {
    let mut env = Env::new();
    let stdin = io::stdin();
    loop {
        print!("> "); io::stdout().flush()?;
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 { break }
        if line.trim() == "exit" { break }
        if line.trim().is_empty() { continue }
        let tokens = tokenize(&line);
        match parse_program(&tokens) {
            Ok(prog) => {
                match eval_program(&prog, &mut env) {
                    Ok(Some(v)) => println!("=> {:?}", v),
                    Ok(None) => println!("=> ()"),
                    Err(e) => println!("Eval error: {:?}", e),
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let mut env = Env::new();
    if args.len() > 1 {
        let path = &args[1];
        let s = fs::read_to_string(path)?;
        let tokens = tokenize(&s);
        match parse_program(&tokens) {
            Ok(prog) => { let _ = eval_program(&prog, &mut env)?; }
            Err(e) => { eprintln!("Parse error: {}", e); }
        }
    } else {
        println!("blk-proto REPL (type 'exit' to quit)");
        repl()?;
    }
    Ok(())
}
