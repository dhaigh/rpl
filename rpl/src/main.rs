mod interpreter;

use interpreter::interp::eval_source;

pub fn main() {
    let source = "(3 4  9 ⊣ 0) ⌈ 10 3 2 ";
    match eval_source(source) {
        Ok(result) => {
            println!("{:?}", result)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
