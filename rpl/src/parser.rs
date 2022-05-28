use crate::scanner::{Op, Token};
use std::fmt;

pub enum Expr<'a> {
    Number(i32),
    Diadic {
        left: Box<Expr<'a>>,
        infix: &'a Op,
        right: Box<Expr<'a>>,
    },
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(value) => write!(f, "{}", value),
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
        let tok = self.consume_token(index);

        if let Some(&Token::Number(n)) = tok {
            let num = Expr::Number(n);

            if *index == self.tokens.len() {
                return Ok(num);
            }

            let infix = self.tokens.get(*index);
            *index += 1;

            match infix {
                Some(infix) => match infix {
                    Token::Operator(op) => {
                        let right = self.p(index);
                        match right {
                            Ok(right) => Ok(Expr::Diadic {
                                left: Box::new(num),
                                infix: op,
                                right: Box::new(right),
                            }),
                            Err(right) => Err(right),
                        }
                    }
                    _ => Err("expected operator"),
                },
                None => Err("expected infix operator"),
            }
        } else {
            return Err("expected number");
        }
    }

    fn consume_token(&self, index: &mut usize) -> Option<&Token> {
        let token = self.tokens.get(*index);
        *index += 1;
        token
    }
}

// pub fn parse(source: &str) -> Result<Expr, &'static str> {
//     let mut scanner = Scanner::new(source);
//     scanner.tokenize();

//     let parser = Parser::new(scanner.tokens);
//     return parser.parse();
// }
