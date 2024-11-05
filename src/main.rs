mod lisp;
mod parser;
use lisp::eval_prelude;
use parser::ExpressionStream;

fn main() {
    let program = "((lambda (x y) (+ (if (< x 10) (+ x 10) x) y)) 2 20)";

    for r in ExpressionStream::from_char_stream(program.chars()) {
        match r {
            Err(err) => println!("ParserError: {:?}", err),
            Ok(expr) => println!(
                "{:?} \n vvvvvvvvvvv \n {:?}\n",
                expr.clone(),
                eval_prelude(expr)
            ),
        }
    }

    println!("Interpreter Done!");
}
