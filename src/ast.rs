use crate::parser::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    SExpr(SExpr),
    Int(i64),
    Identifier(String),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SExpr {
    pub op: Token,
    pub args: Vec<Expr>,
}
