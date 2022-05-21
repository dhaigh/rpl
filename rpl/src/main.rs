mod scanner;
use scanner::Scanner;

pub fn main() {
    let mut s = Scanner::new("12 × 3 + 1");
    s.tokenize();
    for t in s.tokens {
        println!("{}", t);
    }
}
