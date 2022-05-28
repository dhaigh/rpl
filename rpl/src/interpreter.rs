use crate::parser::Expr;
use crate::scanner::Op;

pub fn interpret(expr: Expr) -> i32 {
    match expr {
        Expr::Number(value) => value,
        Expr::Diadic { left, infix, right } => match infix {
            Op::Plus => interpret(*left) + interpret(*right),
            Op::Minus => interpret(*left) - interpret(*right),
            Op::Times => interpret(*left) * interpret(*right),
        },
    }
}
