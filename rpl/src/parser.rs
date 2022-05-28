use crate::scanner::{Op, Token};
use std::fmt;

pub enum Expr<'a> {
    Number(Vec<i32>),
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
        let mut array: Vec<i32> = vec![];

        while let Some(&Token::Number(n)) = self.tokens.get(*index) {
            array.push(n);
            *index += 1;
        }

        if *index == self.tokens.len() {
            return Ok(Expr::Number(array));
        }

        let infix = self.tokens.get(*index);

        if let Some(infix) = infix {
            *index += 1;
            match infix {
                Token::Operator(op) => match self.p(index) {
                    Ok(right) => Ok(Expr::Diadic {
                        left: Box::new(Expr::Number(array)),
                        infix: op,
                        right: Box::new(right),
                    }),
                    Err(e) => Err(e),
                },
                Token::Number(_) => Err("expected infix operator, saw number"),
            }
        } else {
            Err("expected infix operator")
        }
    }
}

// pub fn parse(source: &str) -> Result<Expr, &'static str> {
//     let mut scanner = Scanner::new(source);
//     scanner.tokenize();

//     let parser = Parser::new(scanner.tokens);
//     return parser.parse();
// }
