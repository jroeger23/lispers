use lispers_core::lisp::{eval, Environment};
use lispers_core::parser::ExpressionStream;

fn main() {
    let programs = [
        "((lambda (x y) (+ (if (< x 10) (* x 11) x) y)) 2 20)",
        "(set 'myvar \"hello world!\")",
        "(print myvar) (print 'myvar)",
        "(car (cons 'a 'b)) (cdr (cons 'c 'd)) (cons 'a 'b)",
        "(eval (car (cons 'myvar 'b)))",
        "(set 'pow (lambda (a b) (if (= b 0) 1 (* a (pow a (- b 1))))))",
        "pow",
        "(pow 2 10)",
        "(let '((fib . (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))) (fib 10))",
        "(defun do-n-times (f n) (if (= n 0) '() (cons (f) (do-n-times f (- n 1)))))",
        "(do-n-times (lambda () (print 'hello)) 5)",
        "(progn (print 'hello) (print 'world))",
    ];

    let environment = Environment::default();

    for r in ExpressionStream::from_char_stream(programs.iter().map(|p| p.chars()).flatten()) {
        match r {
            Err(err) => {
                println!("ParserError: {:?}", err);
                break;
            }
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
}
