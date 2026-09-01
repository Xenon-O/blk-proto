use crate::ast::*;
use crate::lexer::Token;

pub fn parse_program(tokens: &[Token]) -> Result<Program, String> {
    let mut idx = 0;
    let mut stmts = Vec::new();

    while idx < tokens.len() {
        match &tokens[idx] {
            Token::EOF => break,
            _ => {
                let (expr, ni) = parse_expr(tokens, idx)?;
                idx = ni;
                stmts.push(Stmt::Expr(expr));
                // consume comma as sequence separator if present
                if let Some(Token::Comma) = tokens.get(idx) {
                    idx += 1;
                }
            }
        }
    }

    Ok(Program { stmts })
}

fn parse_expr(tokens: &[Token], mut idx: usize) -> Result<(Expr, usize), String> {
    match tokens.get(idx) {
        Some(Token::Ident(name)) => {
            // could be assignment if next token is Equal or LessDash
            if let Some(Token::Equal) = tokens.get(idx + 1) {
                // a = expr
                let (rhs, ni) = parse_expr(tokens, idx + 2)?;
                return Ok((Expr::Assign(name.clone(), Box::new(rhs)), ni));
            }
            // identifier expression
            Ok((Expr::Ident(name.clone()), idx + 1))
        }
        Some(Token::Number(n)) => Ok((Expr::Number(*n), idx + 1)),
        Some(Token::LParen) => {
            // parse inner until RParen
            idx += 1;
            // detect whether we have comma-separated or semicolon-separated items
            let mut items = Vec::new();
            let mut sep_is_semicolon = false;
            loop {
                if let Some(Token::RParen) = tokens.get(idx) { idx += 1; break; }
                let (e, ni) = parse_expr(tokens, idx)?;
                items.push(e);
                idx = ni;
                match tokens.get(idx) {
                    Some(Token::Comma) => { idx += 1; }
                    Some(Token::Semicolon) => { sep_is_semicolon = true; idx += 1; }
                    Some(Token::RParen) => { idx += 1; break; }
                    other => return Err("Unexpected token in paren".into()),
                }
            }
            if sep_is_semicolon {
                Ok((Expr::MappingArgs(items), idx))
            } else {
                Ok((Expr::Tuple(items), idx))
            }
        }
        Some(Token::LBrace) => {
            // simple mapping literal parser: {(k1, k2) -> v, ...}
            idx += 1;
            let mut entries = Vec::new();
            loop {
                if let Some(Token::RBrace) = tokens.get(idx) { idx += 1; break; }
                // expect ( ... ) as key
                if let Some(Token::LParen) = tokens.get(idx) {
                    let (key_expr, ni) = parse_expr(tokens, idx)?;
                    idx = ni;
                    // expect Arrow
                    if let Some(Token::Arrow) = tokens.get(idx) { idx += 1; }
                    else { return Err("Expected -> in mapping".into()); }
                    let (val_expr, ni2) = parse_expr(tokens, idx)?;
                    idx = ni2;
                    entries.push((key_expr, val_expr));
                    if let Some(Token::Comma) = tokens.get(idx) { idx += 1; }
                } else { return Err("Expected (key) in mapping".into()); }
            }
            Ok((Expr::MappingLiteral(entries), idx))
        }
        Some(Token::String(s)) => Ok((Expr::String(s.clone()), idx + 1)),
        Some(Token::BacktickBlock(tag, content)) => Ok((Expr::Backtick(tag.clone(), content.clone()), idx + 1)),
        Some(Token::Underscore) => Ok((Expr::Wildcard, idx + 1)),
        Some(Token::Comma) => Err("Unexpected comma".into()),
        Some(Token::Semicolon) => Err("Unexpected semicolon".into()),
        Some(Token::RParen) | Some(Token::RBrace) => Err("Unmatched closing".into()),
        Some(Token::Dot) => Err("Unexpected dot".into()),
        other => Err(format!("Unexpected token: {:?}", other)),
    }
}
