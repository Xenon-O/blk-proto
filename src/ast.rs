#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Number(i64),
    String(String),
    Tuple(Vec<Expr>),
    MappingArgs(Vec<Expr>), // items separated by semicolon in parens
    MappingLiteral(Vec<(Expr, Expr)>),
    Assign(String, Box<Expr>),
    Backtick(String, String),
    Wildcard,
}
