use super::parser::{Array, Expr, Num, Parser};
use super::scanner::{Op, Scanner};

type Monadic = fn(Num) -> Num;
type Diadic = fn(Num, Num) -> Num;

fn entrywise(left: Array, right: Array) -> impl Fn(Diadic, Monadic) -> Array {
    let l = left.len();
    let r = right.len();

    return move |diadic, monadic| match (l, r) {
        (0, _) => right.iter().map(|x| monadic(*x)).collect(),
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

fn plus(left: Array, right: Array) -> Array {
    entrywise(left, right)(|l, r| l + r, |x| x)
}

fn minus(left: Array, right: Array) -> Array {
    entrywise(left, right)(|l, r| l - r, |x| -x)
}

fn times(left: Array, right: Array) -> Array {
    entrywise(left, right)(|l, r| l * r, |x| x)
}

fn divide(left: Array, right: Array) -> Array {
    entrywise(left, right)(|l, r| l / r, |x| x)
}

fn ceil(left: Array, right: Array) -> Array {
    entrywise(left, right)(|l, r| if l > r { l } else { r }, |x| x)
}

fn floor(left: Array, right: Array) -> Array {
    entrywise(left, right)(|l, r| if l < r { l } else { r }, |x| x)
}

enum Tack {
    Left,
    Right,
}

fn tack(side: Tack, left: Array, right: Array) -> Array {
    if left.len() == 0 {
        return right;
    }

    match side {
        Tack::Left => left,
        Tack::Right => right,
    }
}

fn replicate(_left: Array, _right: Array) -> Array {
    // todo
    return vec![];
}

pub fn eval(expr: Expr) -> Array {
    match expr {
        Expr::Number(value) => value,
        Expr::Negative(value) => times(eval(*value), vec![-1]),
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

pub fn eval_source(source: &str) -> Result<Array, &'static str> {
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
