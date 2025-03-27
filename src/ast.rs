use crate::parser::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    SExpr(SExpr),
    Int(i64),
    Identifier(String),
    Bool(bool),
    Float(f64),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SExpr {
    pub op: Token,
    pub args: Vec<Expr>,
}
