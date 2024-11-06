mod lisp;
mod parser;
use parser::ExpressionStream;

use crate::lisp::{eval, Environment};

fn main() {
    let program1 = "((lambda (x y) (+ (if (< x 10) (* x 11) x) y)) 2 20)";
    let program2 = "(set 'myvar \"hello world!\")";
    let program3 = "(print myvar) (print 'myvar)";

    let environment = Environment::default();

    for r in ExpressionStream::from_char_stream(
        program1
            .chars()
            .chain(program2.chars())
            .chain(program3.chars()),
    ) {
        match r {
            Err(err) => println!("ParserError: {:?}", err),
            Ok(expr) => {
                println!("Evaluating: {}", expr.clone());
                match eval(&environment, expr) {
                    Ok(e) => println!("=> {}", e),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }

    println!("Interpreter Done!");
    println!("Environment: {:?}", environment);
}
