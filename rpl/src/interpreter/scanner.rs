use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

fn is_number(string: &str) -> bool {
    if string.len() > 1 {
        return false;
    }

    if let Some(char) = string.chars().next() {
        return match char {
            '0'..='9' => true,
            _ => false,
        };
    }

    false
}

pub enum Op {
    Plus,
    Minus,
    Times,
    Divide,
    Floor,
    Ceil,
    LeftTack,
    RightTack,
    Replicate,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Op::Plus => "+",
                Op::Minus => "-",
                Op::Times => "×",
                Op::Divide => "÷",
                Op::Floor => "⌊",
                Op::Ceil => "⌈",
                Op::LeftTack => "⊣",
                Op::RightTack => "⊢",
                Op::Replicate => "/",
            }
        )
    }
}

pub enum Token {
    Number(String),
    HighMinus,
    Operator(Op), // todo: rename to function
    LeftParen,
    RightParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::HighMinus => write!(f, "¯"),
            Token::Number(value) => write!(f, "{}", value),
            Token::Operator(op) => write!(f, "{}", op),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}

pub struct Scanner<'a> {
    pub index: usize,
    pub graphemes: Vec<&'a str>,
    pub tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let tokens: Vec<Token> = vec![];

        Self {
            index: 0,
            graphemes: source.graphemes(true).collect::<Vec<&str>>(),
            tokens,
        }
    }

    pub fn tokenize(&mut self) {
        while let Some(c) = self.current_grapheme() {
            if is_number(c) {
                self.consume_number();
            } else {
                match c {
                    &" " => {}
                    &"¯" => self.tokens.push(Token::HighMinus),
                    &"+" => self.tokens.push(Token::Operator(Op::Plus)),
                    &"-" => self.tokens.push(Token::Operator(Op::Minus)),
                    &"×" => self.tokens.push(Token::Operator(Op::Times)),
                    &"÷" => self.tokens.push(Token::Operator(Op::Divide)),
                    &"⌊" => self.tokens.push(Token::Operator(Op::Floor)),
                    &"⌈" => self.tokens.push(Token::Operator(Op::Ceil)),
                    &"⊣" => self.tokens.push(Token::Operator(Op::LeftTack)),
                    &"⊢" => self.tokens.push(Token::Operator(Op::RightTack)),
                    &"/" => self.tokens.push(Token::Operator(Op::Replicate)),
                    &"(" => self.tokens.push(Token::LeftParen),
                    &")" => self.tokens.push(Token::RightParen),
                    _ => {
                        println!("unknown character `{}`", c);
                    }
                }
                self.advance();
            }
        }
    }

    fn consume_number(&mut self) {
        let start = self.index;

        while let Some(g) = self.current_grapheme() {
            if is_number(g) {
                self.advance();
            } else {
                break;
            }
        }

        let mut s = String::with_capacity(self.index - start);
        for i in start..self.index {
            s.push_str(self.graphemes[i]);
        }

        self.tokens.push(Token::Number(s));
    }

    fn current_grapheme(&self) -> Option<&&str> {
        self.graphemes.get(self.index)
    }

    fn advance(&mut self) -> Option<&&str> {
        let grapheme = self.graphemes.get(self.index);
        self.index += 1;
        grapheme
    }
}
