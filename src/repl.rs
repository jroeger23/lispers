use lisp::Expression;
use parser::ParserError;

use std::io::Write;
mod parser;
mod lisp;

fn main() {
    let env = lisp::Environment::default();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            println!("Exiting REPL...");
            break;
        }

        match parser::ExpressionStream::from_char_stream(input.chars()).collect::<Result<Vec<Expression>, ParserError>>() {
            Err(e) => println!("Parser Error: {:?}", e),
            Ok(exprs) => {
                for expr in exprs {
                    match lisp::eval(&env, expr) {
                        Err(e) => println!("Eval Error: {}", e),
                        Ok(val) => println!("{}", val),
                    }
                }
            }
        }
    }
}
