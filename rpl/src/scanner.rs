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

pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Times,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::Number(value) => write!(f, "{}", value),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Times => write!(f, "×"),
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
                    &"+" => self.tokens.push(Token::Plus),
                    &"-" => self.tokens.push(Token::Minus),
                    &"×" => self.tokens.push(Token::Times),
                    &" " => {}
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

        match s.parse::<i32>() {
            Ok(n) => {
                let num = Token::Number(n);
                self.tokens.push(num);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
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
