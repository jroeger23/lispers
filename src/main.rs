mod lisp;
mod parser;
use parser::ExpressionStream;

use crate::lisp::{eval, Environment};

fn main() {
    let program1 = "((lambda (x y) (+ (if (< x 10) (* x 11) x) y)) 2 20)";
    let program2 = "(set myvar \"hello world!\")";
    let program3 = "myvar";

    let mut environment = Environment::default();

    for r in ExpressionStream::from_char_stream(
        program1
            .chars()
            .chain(program2.chars())
            .chain(program3.chars()),
    ) {
        match r {
            Err(err) => println!("ParserError: {:?}", err),
            Ok(expr) => println!(
                "{:?} \n vvvvvvvvvvv \n {:?}\n",
                expr.clone(),
                eval(&environment, expr)
            ),
        }
    }

    println!("Interpreter Done!");
    println!("Environment: {:?}", environment);
}
