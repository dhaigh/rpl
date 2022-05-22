mod parser;
mod scanner;
use parser::parse;

pub fn main() {
    let tree = parse("12 × 3 + 1");
    println!("{}", tree);
}
