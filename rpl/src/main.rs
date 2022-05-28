mod interpreter;
mod parser;
mod scanner;
use interpreter::interpret;
use parser::Parser;
use scanner::Scanner;

pub fn main() {
    let source = "12 × 3 + 1";

    let mut scanner = Scanner::new(source);
    scanner.tokenize();

    let parser = Parser::new(scanner.tokens);

    if let Ok(tree) = parser.parse() {
        println!("> {}", tree);
        let result = interpret(tree);
        println!("{}", result)
    }
}
