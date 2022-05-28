use crate::parser::Expr;
use crate::scanner::Op;

enum Tack {
    Left,
    Right,
}

fn entrywise(left: Vec<i32>, right: Vec<i32>) -> impl Fn(fn(i32, i32) -> i32) -> Vec<i32> {
    let l = left.len();
    let r = right.len();

    return move |func| match (l, r) {
        (0, _) => right.clone(),
        (_, 0) => left.clone(),
        (1, _) => right.iter().map(|x| func(left[0], *x)).collect(),
        (_, 1) => left.iter().map(|x| func(right[0], *x)).collect(),
        _ => {
            if l == r {
                return left
                    .iter()
                    .zip(right.iter())
                    .map(|(a, b)| func(*a, *b))
                    .collect();
            } else {
                panic!("mismatching arg length ({}, {})", l, r);
            }
        }
    };
}

fn plus(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l + r)
}

fn minus(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l - r)
}

fn times(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l * r)
}

fn divide(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l / r)
}

fn ceil(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| if l > r { l } else { r })
}

fn floor(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| if l < r { l } else { r })
}

fn tack(side: Tack, left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    if left.len() == 0 {
        return right;
    }

    match side {
        Tack::Left => left,
        Tack::Right => right,
    }
}

fn replicate(_left: Vec<i32>, _right: Vec<i32>) -> Vec<i32> {
    // todo
    return vec![];
}

pub fn interpret(expr: Expr) -> Vec<i32> {
    match expr {
        Expr::Number(value) => value,
        Expr::Diadic { left, infix, right } => {
            let l = interpret(*left);
            let r = interpret(*right);

            if r.len() == 0 {
                panic!("no right");
            }

            match infix {
                Op::Plus => plus(l, r),
                Op::Minus => minus(l, r),
                Op::Times => times(l, r),
                Op::Divide => divide(l, r),
                Op::Ceil => ceil(l, r),
                Op::Floor => floor(l, r),
                Op::LeftTack => tack(Tack::Left, l, r),
                Op::RightTack => tack(Tack::Right, l, r),
                Op::Replicate => replicate(l, r),
            }
        }
    }
}
