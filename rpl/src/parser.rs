use crate::scanner::{Scanner, Token};

struct Expr {
    num: Option<i32>,
    diadic: Option<Box<Diadic>>,
}

pub struct Diadic {
    pub left: Expr,
    pub infix: Token,
    pub right: Expr,
}

impl Diadic {
    pub fn new(left: Expr, infix: Token, right: Expr) -> Self {
        Self { left, infix, right }
    }
}

impl Expr {
    pub fn new_num(num: i32) -> Self {
        Self {
            num: Some(num),
            diadic: None,
        }
    }

    pub fn new_diadic(left: Expr, infix: Token, right: Expr) -> Self {
        let diadic = Diadic::new(left, infix, right);
        Self {
            num: None,
            diadic: Some(Box::new(diadic)),
        }
    }
}

/**
 * expr → (term OP)* term
 * term → NUMBER (" " NUMBER)*
 */

pub struct Parser {
    pub index: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { index: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        let tok = self.consume_token();
        if let Some(&Token::Number(n)) = tok {
            let num = Expr::new_num(n);
            let infix = self.consume_token();

            match infix {
                Some(infix) => {
                    let right = self.parse();
                    match right {
                        Ok(right) => Ok(Expr::new_diadic(num, infix, right)),
                        Err(right) => Err(right),
                    }
                }
                None => Err("expected infix operator"),
            }
        } else {
            return Err("expected number");
        }
    }

    fn consume_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}

pub fn parse(source: &str) -> &str {
    let mut scanner = Scanner::new(source);
    scanner.tokenize();
    let mut parser = Parser::new(scanner.tokens);
    parser.parse();
    return "hello";
}
