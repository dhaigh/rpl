use super::scanner::{Op, Token};
use std::fmt;

pub type Num = f64;
pub type Array = Vec<Num>;

pub enum Expr<'a> {
    Number(Array),
    Diadic {
        left: Box<Expr<'a>>,
        infix: &'a Op,
        right: Box<Expr<'a>>,
    },
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(value) => write!(f, "{:?}", value),
            Expr::Diadic { left, infix, right } => write!(f, "({} {} {})", left, infix, right),
        }
    }
}

/**
 * expr → (term OP)* term
 * term → NUMBER (" " NUMBER)*
 */

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Expr, &'static str> {
        let mut index: usize = 0;
        return self.p(&mut index);
    }

    fn p(&self, index: &mut usize) -> Result<Expr, &'static str> {
        let mut array: Array = vec![];

        while let Some(Token::Number(n)) = self.tokens.get(*index) {
            match n.parse::<Num>() {
                Ok(n) => {
                    array.push(n);
                }
                Err(_) => {
                    return Err("couldn't parse float for some reason");
                }
            }
            *index += 1;
        }

        let mut expr: Result<Expr, &'static str> = Ok(Expr::Number(array));

        while let Some(token) = self.tokens.get(*index) {
            *index += 1;
            expr = match token {
                Token::Operator(infix) => match self.p(index) {
                    Ok(right) => match expr {
                        Ok(expr) => Ok(Expr::Diadic {
                            left: Box::new(expr),
                            infix,
                            right: Box::new(right),
                        }),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                },
                Token::Number(_) => Err("unexpected number"),
                Token::LeftParen => self.p(index),
                Token::RightParen => {
                    return expr;
                }
            }
        }

        expr
    }
}

// pub fn parse(source: &str) -> Result<Expr, &'static str> {
//     let mut scanner = Scanner::new(source);
//     scanner.tokenize();

//     let parser = Parser::new(scanner.tokens);
//     return parser.parse();
// }
