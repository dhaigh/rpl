use super::parser::{Expr, Parser};
use super::scanner::{Op, Scanner};

fn entrywise(
    left: Vec<i32>,
    right: Vec<i32>,
) -> impl Fn(fn(i32, i32) -> i32, fn(i32) -> i32) -> Vec<i32> {
    let l = left.len();
    let r = right.len();

    return move |diadic, monadic| match (l, r) {
        (0, _) => right.iter().map(|x| monadic(*x)).collect(),
        (_, 0) => left.iter().map(|x| monadic(*x)).collect(),
        (1, _) => right.iter().map(|x| diadic(left[0], *x)).collect(),
        (_, 1) => left.iter().map(|x| diadic(*x, right[0])).collect(),
        _ => {
            if l == r {
                return left
                    .iter()
                    .zip(right.iter())
                    .map(|(a, b)| diadic(*a, *b))
                    .collect();
            } else {
                panic!("mismatching arg length ({}, {})", l, r);
            }
        }
    };
}

fn plus(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l + r, |x| x)
}

fn minus(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l - r, |x| -x)
}

fn times(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l * r, |x| x)
}

fn divide(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| l / r, |x| x)
}

fn ceil(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| if l > r { l } else { r }, |x| x)
}

fn floor(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    entrywise(left, right)(|l, r| if l < r { l } else { r }, |x| x)
}

enum Tack {
    Left,
    Right,
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

pub fn eval(expr: Expr) -> Vec<i32> {
    match expr {
        Expr::Number(value) => value,
        Expr::Diadic { left, infix, right } => {
            let l = eval(*left);
            let r = eval(*right);

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

pub fn eval_source(source: &str) -> Result<Vec<i32>, &'static str> {
    let mut scanner = Scanner::new(source);
    scanner.tokenize();

    let parser = Parser::new(scanner.tokens);

    match parser.parse() {
        Ok(tree) => {
            println!("> {}", tree);
            Ok(eval(tree))
        }
        Err(e) => Err(e),
    }
}
