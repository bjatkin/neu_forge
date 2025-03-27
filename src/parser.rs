pub mod lexer;

use crate::ast;
use lexer::token::Type;

#[derive(Clone)]
pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let lexer = lexer::Lexer::new(source);
        let parser = Parser { lexer: lexer };
        return parser;
    }

    pub fn parse(&mut self) -> Vec<ast::Expr> {
        let mut exprs = Vec::new();
        loop {
            let expr = self.parse_expr();
            match expr {
                Some(ref e) => exprs.push(e.to_owned()),
                None => break,
            }
        }

        // TODO: type check this before returning it
        // actually can I even do that becuase value(and the associated types) live in the interpreter,
        // not in the complier... Hmmmm... I'll have to think about that...
        return exprs;
    }

    fn parse_expr(&mut self) -> Option<ast::Expr> {
        let tok = self.lexer.peek();
        return match tok.t_type {
            Type::OpenParen => {
                return match self.parse_sexpr() {
                    Some(e) => Some(ast::Expr::SExpr(e)),
                    None => Some(ast::Expr::Empty),
                };
            }
            Type::Int => {
                let tok = self.lexer.take();
                let i = match tok.value.parse() {
                    Ok(i) => i,
                    Err(e) => {
                        let value = tok.value;
                        panic!("'{value}' is not a valid value {e}")
                    }
                };
                return Some(ast::Expr::Int(i));
            }
            Type::Float => {
                let tok = self.lexer.take();
                let f = match tok.value.parse() {
                    Ok(f) => f,
                    Err(e) => {
                        let value = tok.value;
                        panic!("'{value}' is not a valid value {e}")
                    }
                };
                return Some(ast::Expr::Float(f));
            }
            Type::Bool => {
                let tok = self.lexer.take();
                let b = match tok.value.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => {
                        let value = tok.value;
                        panic!("'{value}' invalid boolean value")
                    }
                };
                return Some(ast::Expr::Bool(b));
            }
            Type::Identifier => {
                let ident = self.lexer.take();
                return Some(ast::Expr::Identifier(ident.value));
            }
            Type::EOF => None,
            _ => {
                let t_type = tok.t_type;
                panic!("failed to parse unknown token type {t_type:#?}")
            }
        };
    }

    fn parse_sexpr(&mut self) -> Option<ast::SExpr> {
        // take the open paraen before parsing the s-expr
        match self.lexer.take().t_type {
            Type::OpenParen => { /* ok */ }
            _ => panic!("s-expr must start with an open paren"),
        }

        // get the expr operator
        let op = self.lexer.take();

        if op.t_type == Type::CloseParen {
            // this is actually an empty value
            return None;
        }

        let mut args = Vec::new();
        while self.lexer.peek().t_type != Type::CloseParen {
            if self.lexer.peek().t_type == Type::EOF {
                panic!("unclosed s-expr!")
            }

            match self.parse_expr() {
                Some(e) => args.push(e),
                None => break,
            }
        }

        // take the closing paren before returning
        _ = self.lexer.take();

        return Some(ast::SExpr { op: op, args: args });
    }
}
