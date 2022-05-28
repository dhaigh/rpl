use crate::parser::Expr;
use crate::scanner::Token;

pub fn interpret(expr: Expr) -> i32 {
    match expr {
        Expr::Number(value) => value,
        Expr::Diadic { left, infix, right } => match infix {
            Token::Plus => interpret(*left) + interpret(*right),
            Token::Minus => interpret(*left) - interpret(*right),
            Token::Times => interpret(*left) * interpret(*right),
            _ => panic!("wtf"),
        },
    }
}
