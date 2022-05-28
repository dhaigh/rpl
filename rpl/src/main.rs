mod interpreter;
mod parser;
mod scanner;
use interpreter::interpret;
use parser::Parser;
use scanner::Scanner;

pub fn main() {
    let source = "1 2 3 × 4 5 6";

    let mut scanner = Scanner::new(source);
    scanner.tokenize();

    let parser = Parser::new(scanner.tokens);

    match parser.parse() {
        Ok(tree) => {
            println!("> {}", tree);
            let result = interpret(tree);
            println!("{:?}", result)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
