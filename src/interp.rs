mod value;

use crate::ast::{self, Expr};
use crate::parser::lexer::token::Type;
use std::collections::HashMap;
use value::Value;

pub struct Interp {
    identifiers: HashMap<String, Value>,
}

impl Interp {
    pub fn new() -> Self {
        return Interp {
            identifiers: HashMap::new(),
        };
    }

    pub fn interp(&mut self, exprs: Vec<ast::Expr>) -> Value {
        let mut value: Value = Value::None;

        for e in exprs.as_slice() {
            value = self.interp_expr(e);
        }

        return value;
    }

    pub fn interp_expr(&mut self, expr: &ast::Expr) -> Value {
        match expr {
            Expr::SExpr(e) => self.interp_sexpr(e),
            Expr::Int(i) => Value::Int(*i),
            Expr::Identifier(name) => match self.identifiers.get(name) {
                Some(v) => v.clone(),
                None => panic!("unknown identifier '{name}'"),
            },
            Expr::Empty => Value::None,
        }
    }

    fn interp_sexpr(&mut self, sexpr: &ast::SExpr) -> Value {
        match sexpr.op.t_type {
            Type::Plus => {
                // TODO: I can probably do some sort of map function here for all these
                // math operators
                if sexpr.args.len() == 0 {
                    return Value::None;
                }

                let mut sum = 0;
                for e in sexpr.args.as_slice() {
                    let arg = self.interp_expr(e);
                    match arg {
                        Value::Int(i) => sum += i,
                        Value::None => panic!("can not add empty value"),
                    }
                }
                return Value::Int(sum);
            }
            Type::Minus => {
                let first = match sexpr.args.first() {
                    Some(i) => self.interp_expr(i),
                    None => return Value::None,
                };
                let mut total = match first {
                    Value::Int(i) => i,
                    Value::None => panic!("can not add empty value"),
                };

                for e in &sexpr.args.as_slice()[1..] {
                    let arg = self.interp_expr(e);
                    match arg {
                        Value::Int(i) => total -= i,
                        Value::None => panic!("can not add empty value"),
                    }
                }

                return Value::Int(total);
            }
            Type::Multiply => {
                let first = match sexpr.args.first() {
                    Some(i) => self.interp_expr(i),
                    None => return Value::None,
                };
                let mut total = match first {
                    Value::Int(i) => i,
                    Value::None => panic!("can not add empty value"),
                };

                for e in &sexpr.args.as_slice()[1..] {
                    let arg = self.interp_expr(e);
                    match arg {
                        Value::Int(i) => total *= i,
                        Value::None => panic!("can not add empty value"),
                    }
                }

                return Value::Int(total);
            }
            Type::Divide => {
                let first = match sexpr.args.first() {
                    Some(i) => self.interp_expr(i),
                    None => return Value::None,
                };
                let mut total = match first {
                    Value::Int(i) => i,
                    Value::None => panic!("can not add empty value"),
                };

                for e in &sexpr.args.as_slice()[1..] {
                    let arg = self.interp_expr(e);
                    match arg {
                        Value::Int(i) => total /= i,
                        Value::None => panic!("can not add empty value"),
                    }
                }

                return Value::Int(total);
            }
            Type::Let => {
                let ident = match sexpr.args.first() {
                    Some(e) => match e {
                        Expr::Identifier(i) => i,
                        _ => panic!("first argument in let expression must be an identifier"),
                    },
                    None => panic!("let requires an identifier as it's first argument"),
                };

                if sexpr.args.len() != 2 {
                    panic!("let expression takes exactly two arguments")
                }

                let value = self.interp_expr(&sexpr.args.as_slice()[1]);

                self.identifiers.insert(ident.clone(), value);

                return Value::None;
            }
            _ => {
                let op = &sexpr.op.value;
                let t_type = &sexpr.op.t_type;
                panic!("unknown operator '{op}' with type '{t_type:#?}'");
            }
        }
    }
}
