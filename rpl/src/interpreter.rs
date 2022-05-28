use crate::parser::Expr;
use crate::scanner::Op;

fn entrywise(left: Vec<i32>, right: Vec<i32>) -> impl Fn(fn(i32, i32) -> i32) -> Vec<i32> {
    if left.len() != right.len() {
        panic!("mismatching arg length ({}, {})", left.len(), right.len());
    }

    return move |func| {
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| func(*a, *b))
            .collect()
    };
}

fn plus(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    return entrywise(left, right)(|l, r| l + r);
}

fn minus(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    return entrywise(left, right)(|l, r| l - r);
}

fn times(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    return entrywise(left, right)(|l, r| l * r);
}

fn divide(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    return entrywise(left, right)(|l, r| l / r);
}

pub fn interpret(expr: Expr) -> Vec<i32> {
    match expr {
        Expr::Number(value) => value,
        Expr::Diadic { left, infix, right } => {
            let l = interpret(*left);
            let r = interpret(*right);

            match infix {
                Op::Plus => plus(l, r),
                Op::Minus => minus(l, r),
                Op::Times => times(l, r),
                Op::Divide => divide(l, r),
            }
        }
    }
}
