use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

pub enum Token {
    Number(i32),
    Plus,
    Times,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::Number(value) => write!(f, "{}", value),
            Token::Plus => write!(f, "+"),
            Token::Times => write!(f, "×"),
        }
    }
}

pub struct Scanner<'a> {
    pub index: usize,
    pub graphemes: Vec<&'a str>,
    pub tokens: Vec<Token>,
}

fn is_number(string: &str) -> bool {
    if string.len() > 1 {
        return false;
    }

    return match string.chars().next() {
        Some(c) => '0' <= c && c <= '9',
        _ => false,
    };
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
        while let Some(c) = self.next_grapheme() {
            if is_number(c) {
                self.consume_number();
            } else {
                match c {
                    &"+" => self.tokens.push(Token::Plus),
                    &"×" => self.tokens.push(Token::Times),
                    &" " => {}
                    _ => {}
                }
            }
        }
    }

    fn consume_number(&mut self) {
        let start = self.index;

        while let Some(g) = self.current_grapheme() {
            if !is_number(g) {
                break;
            }
            self.index += 1;
        }

        let mut s = String::with_capacity(self.index - start);
        for i in start..self.index {
            s.push_str(self.graphemes[i]);
        }

        match s.parse::<i32>() {
            Ok(n) => self.tokens.push(Token::Number(n)),
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    fn current_grapheme(&self) -> Option<&&str> {
        self.graphemes.get(self.index)
    }

    fn next_grapheme(&mut self) -> Option<&&str> {
        self.index += 1;
        self.current_grapheme()
    }
}
